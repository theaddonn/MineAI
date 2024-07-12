#include "dllmain.hpp"
#include <thread>

ModFunction void Initialize(AmethystContext* ctx) 
{
	Log::Info("Hai!!!");

	// Create a new thread for mine ai
	std::thread mine_ai_thread(MineAIThread);

	// Spawn the new thread
	mine_ai_thread.detach();
}


void MineAIThread() {
	Log::Info("[MineAI] [INIT] Started Initialising");

	MineAIInitResult init_result = mine_ai_init(MineAIInitOptions{
		MineAIInitDevice::CudaIfAvailable,
		50,
		// Negative numbers set internal Rust code option to None
		-1,
		});

	if (init_result.mError != nullptr) {
		Log::Info("[MineAI] [INIT] Error while Initialising: \"{}\"", *init_result.mError);

		return;
	}

	Log::Info("[MineAI] [INIT] Initilized successfully");
	MineAIObject object = *init_result.mObject;



	Log::Info("[MineAI] [PROMPT] Prompted AI: \"What is your name?\"");

	MineAIGenerationResult generation_result = mine_ai_generate(object, std::string("What is your name?"));


	if (generation_result.mError != nullptr) {
		Log::Info("[MineAI] [PROMPT] Error while generating response: \"{}\"", *generation_result.mError);
	}
	else
	{
		Log::Info("[MineAI] [PROMPT] Prompt result: \"{}\"", *generation_result.mText);
	}

}
