// Please install OpenAI SDK first: `npm install openai`
//deno --allow-env --allow-net E:\app\julia\wfs2map\src\energy-monitor-tauri\src\openai.ts

//import OpenAI from "openai";
import OpenAI from 'jsr:@openai/openai';

const openai = new OpenAI({
  baseURL: 'https://api.deepseek.com',
  apiKey: 'sk-03e8a187df9e40e99b51ee7a41ece771'
});

const system_prompt = `
The user will provide some exam text. Please parse the "question" and "answer" and output them in JSON format. 

EXAMPLE INPUT: 
Which is the highest mountain in the world? Mount Everest.

EXAMPLE JSON OUTPUT:
{
    "question": "Which is the highest mountain in the world?",
    "answer": "Mount Everest"
}
`;

const user_prompt = "Which is the longest river in the world? The Nile River."

let messages = [{"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}]

async function main() {
  const completion = await openai.chat.completions.create({
    messages: [{ role: "system", content: "You are a helpful assistant." }],
    model: "deepseek-chat",
  });

  console.log(completion.choices[0].message.content);
}

//main();
