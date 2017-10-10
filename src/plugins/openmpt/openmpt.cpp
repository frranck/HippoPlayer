#include <libopenmpt/libopenmpt.hpp>

#include "../../plugin_api/HippoPlugin.h"

int dummy() { return 0; }

struct OpenMptData
{
    openmpt::module* mod = 0;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* openMptInfo(void* userData)
{
	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* openMptTrackInfo(void* userData)
{
	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char** openMptSupportedExtensions(void* userData)
{
	static const char* supportedFomats[] =
	{
		"ahx",
		"hvl",
		0,
	};

	return supportedFomats;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void* openMptCreate()
{
	// TODO: supply custom allocator

	void* replayerData = malloc(sizeof(struct OpenMptData));
	memset(replayerData, 0, sizeof(struct OpenMptData));

	return replayerData;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int openMptDestroy(void* userData)
{
	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int openMptOpen(void* userData, const char* buffer)
{
	// TODO: Add reader functions etc to be used instead of fopen as file may come from zip, etc

	FILE* file = fopen(buffer, "rb");
	fseek(file, 0, SEEK_END);
	size_t size = ftell(file);
	fseek(file, 0, SEEK_SET);
	uint8_t* data = new uint8_t[size];
	fread(data, size, 1, file);
	fclose(file);

	struct OpenMptData* replayerData = (struct OpenMptData*)userData;
    replayerData->mod = new openmpt::module(data, size);

	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int openMptClose(void* userData)
{
	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int openMptFrameSize(void* userData)
{
	return 480;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int openMptReadData(void* userData, void* dest)
{
	struct OpenMptData* replayerData = (struct OpenMptData*)userData;
    int t = replayerData->mod->read_interleaved_stereo(48000, openMptFrameSize(userData), (int16_t*)dest);

	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int openMptSeek(void* userData, int ms)
{
	return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static HippoPlaybackPlugin g_openmptPlugin =
{
	1,
	openMptInfo,
	openMptTrackInfo,
	openMptSupportedExtensions,
	openMptCreate,
	openMptDestroy,
	openMptOpen,
	openMptClose,
	openMptReadData,
	openMptSeek,
	openMptFrameSize,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C" HIPPO_EXPORT HippoPlaybackPlugin* getPlugin()
{
	return &g_openmptPlugin;
}


