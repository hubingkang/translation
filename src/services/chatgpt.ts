// import OpenAI from 'openai';
import { streamText as _streamText, convertToCoreMessages, generateText } from 'ai';
import { createOpenAI } from '@ai-sdk/openai';

export function getOpenAIModel(apiKey: string) {
  const openai = createOpenAI({
    apiKey: '',
    baseURL: '',
  });

  return openai('gpt-4-turbo');

  // return openai('claude-3-5-sonnet-20241022', {});
}

export const checkGrammar = async (content: string) => {
  try {
    // const response = await openai.chat.completions.create({
    //   model: "gpt-3.5-turbo",
    //   messages: [
    //     {
    //       role: "system",
    //       content: "You are an English grammar checker. Analyze the following text and provide feedback on any grammar errors. If there are no errors, say so. Be concise and clear."
    //     },
    //     {
    //       role: "user",
    //       content: text
    //     }
    //   ],
    // });

    // return _streamText({
    //   model: getOpenAIModel(getAPIKey(env)),
    //   system: getSystemPrompt(),
    //   maxTokens: '8192',
    //   messages: convertToCoreMessages(messages),
    //   // ...options,
    // });

    console.log('content', content);
    
    const { text } = await generateText({
      model: getOpenAIModel('gpt-4-turbo'),
      prompt: `You are an English grammar checker. Analyze the following text and provide feedback on any grammar errors. If there are no errors, say so. Be concise and clear.\n\n${content}`,
    });

    return text;
    // return response.choices[0].message.content;
  } catch (error) {
    console.error('Error checking grammar:', error);
    throw error;
  }
};
