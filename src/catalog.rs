/// Static catalog: 7 BWOC backends with bilingual descriptions, vendor CLI
/// binary names, and available model lists.  Used by the backend-picker stage.

use crate::i18n::Lang;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backend {
    /// The value passed to `bwoc new --backend` — must match bwoc's registry.
    pub id: &'static str,
    /// Display label shown in the selection list.
    pub label: &'static str,
    /// Vendor CLI binary to probe on PATH (`binary_present`).
    pub binary: &'static str,
    /// Short English description rendered in the right pane.
    pub description_en: &'static str,
    /// Short Thai description rendered in the right pane.
    pub description_th: &'static str,
    /// Available model IDs for this backend.
    pub models: &'static [&'static str],
}

impl Backend {
    pub fn description(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => self.description_en,
            Lang::Th => self.description_th,
        }
    }
}

pub const BACKENDS: &[Backend] = &[
    Backend {
        id: "claude",
        label: "Claude (Anthropic)",
        binary: "claude",
        description_en: "\
Claude is Anthropic's AI, known for careful reasoning and safety.\n\
Great for coding, data analysis, and tasks requiring deep thought.\n\
Requires the 'claude' CLI installed and authenticated (claude login).\n\
Billed per token consumed.",
        description_th: "\
Claude เป็น AI ของ Anthropic ที่ขึ้นชื่อเรื่องความฉลาดและความระมัดระวัง\n\
เหมาะกับงานเขียนโค้ด วิเคราะห์ข้อมูล และงานที่ต้องการเหตุผลลึก\n\
ต้องติดตั้ง CLI 'claude' และล็อกอินก่อน (claude login)\n\
ค่าใช้จ่ายตามปริมาณ token ที่ใช้จริง",
        models: &[
            "claude-opus-4-8",
            "claude-sonnet-4-6",
            "claude-haiku-4-5",
        ],
    },
    Backend {
        id: "antigravity",
        label: "Antigravity (agy)",
        binary: "agy",
        description_en: "\
Antigravity is an AI gateway that unifies multiple providers in one place.\n\
Supports Gemini, Claude, and GPT through the 'agy' CLI.\n\
Requires agy installed and authenticated (agy login).\n\
Ideal if you want to switch models without changing your backend setup.",
        description_th: "\
Antigravity เป็น AI gateway ที่รวมหลาย provider ไว้ในที่เดียว\n\
ใช้ได้ทั้ง Gemini, Claude, และ GPT ผ่าน CLI 'agy'\n\
ต้องติดตั้ง agy และล็อกอินก่อน (agy login)\n\
เหมาะถ้าอยากลองเปลี่ยน model ไปมาโดยไม่ต้องเปลี่ยน backend",
        models: &[
            "gemini-3.5-flash",
            "gemini-3.1-pro",
            "claude-sonnet-4-6-thinking",
            "gpt-oss-120b",
        ],
    },
    Backend {
        id: "codex",
        label: "Codex (OpenAI CLI)",
        binary: "codex",
        description_en: "\
Codex uses OpenAI's GPT-5 models, tuned specifically for code generation.\n\
Requires the 'codex' CLI installed and an OpenAI API key.\n\
Best for software development, bug fixing, and refactoring.\n\
Billed according to OpenAI API usage.",
        description_th: "\
Codex ใช้ model GPT-5 ของ OpenAI ที่เก่งด้านการเขียนโค้ดโดยเฉพาะ\n\
ต้องติดตั้ง CLI 'codex' และมี OpenAI API key\n\
เหมาะกับงานพัฒนาซอฟต์แวร์ แก้ bug และ refactor โค้ด\n\
ค่าใช้จ่ายตาม API usage ของ OpenAI",
        models: &[
            "gpt-5.5",
            "gpt-5.4",
            "gpt-5.4-mini",
            "gpt-5.3-codex",
        ],
    },
    Backend {
        id: "kimi",
        label: "Kimi (Moonshot AI)",
        binary: "kimi",
        description_en: "\
Kimi is an AI from Moonshot AI with an exceptionally large context window.\n\
Ideal for reading long documents or analysing many files at once.\n\
Requires the 'kimi' CLI installed and authenticated.\n\
A strong choice when you need large context at a reasonable price.",
        description_th: "\
Kimi เป็น AI จาก Moonshot AI มี context window ขนาดใหญ่มาก\n\
เหมาะกับงานที่ต้องอ่านเอกสารยาว ๆ หรือวิเคราะห์หลายไฟล์พร้อมกัน\n\
ต้องติดตั้ง CLI 'kimi' และล็อกอินก่อน\n\
ตัวเลือกที่ดีถ้าต้องการ context ขนาดใหญ่ในราคาที่เหมาะสม",
        models: &[
            "kimi-k2",
            "kimi-k1.5",
        ],
    },
    Backend {
        id: "copilot",
        label: "GitHub Copilot",
        binary: "copilot",
        description_en: "\
GitHub Copilot integrates seamlessly with the GitHub ecosystem.\n\
Great for developers already on GitHub with a Copilot subscription.\n\
Requires GitHub CLI (gh) + copilot extension, authenticated.\n\
Supports Claude, GPT-5, and other models depending on your plan.",
        description_th: "\
GitHub Copilot ผสานกับ ecosystem ของ GitHub ได้อย่างลงตัว\n\
เหมาะสำหรับนักพัฒนาที่ใช้ GitHub อยู่แล้วและมี Copilot subscription\n\
ต้องติดตั้ง GitHub CLI (gh) + copilot extension และล็อกอินก่อน\n\
รองรับทั้ง Claude, GPT-5 และ model อื่น ๆ ตาม subscription",
        models: &[
            "claude-sonnet-4-6",
            "claude-haiku-4-5",
            "gpt-5.5",
            "gpt-5.5-codex",
        ],
    },
    Backend {
        id: "ollama",
        label: "Ollama (local, free)",
        binary: "ollama",
        description_en: "\
Ollama runs AI models locally on your machine — no data leaves your device.\n\
100% free, no API costs, works offline; great for sensitive data.\n\
Requires Ollama installed and your chosen model pulled (ollama pull <model>).\n\
Performance depends on your machine's RAM and GPU.",
        description_th: "\
Ollama รัน AI model บนเครื่องของคุณเอง ไม่ต้องส่งข้อมูลออกไปนอก\n\
ฟรี 100% ไม่มีค่า API ทำงานออฟไลน์ได้ เหมาะกับข้อมูลความลับ\n\
ต้องติดตั้ง Ollama และ pull model ที่ต้องการไว้ก่อน (ollama pull <model>)\n\
ประสิทธิภาพขึ้นอยู่กับ RAM และ GPU ของเครื่อง",
        models: &[
            "qwen2.5-coder:7b",
            "llama3.1:8b",
            "mistral-nemo",
            "gemma4:8b",
        ],
    },
    Backend {
        id: "openai-compatible",
        label: "OpenAI-compatible (custom endpoint)",
        binary: "",  // no single binary — uses bwoc-harness directly
        description_en: "\
Works with any API that is compatible with the OpenAI format, such as\n\
LM Studio, vLLM, Together AI, or other self-hosted models.\n\
You must provide the baseUrl of the endpoint (e.g. http://localhost:1234/v1).\n\
BWOC sends requests to that endpoint directly via bwoc-harness.",
        description_th: "\
ใช้ได้กับ API ใด ๆ ที่ compatible กับ OpenAI format เช่น LM Studio,\n\
vLLM, Together AI, หรือ self-hosted model อื่น ๆ\n\
ต้องระบุ baseUrl ของ endpoint (เช่น http://localhost:1234/v1)\n\
BWOC จะส่ง request ไปยัง endpoint นั้นโดยตรงผ่าน bwoc-harness",
        models: &[
            "gpt-5.5",
            "gpt-5.5-pro",
            "gpt-5.4",
            "gpt-5.4-mini",
        ],
    },
];

// ---------------------------------------------------------------------------
// Stage help text (bilingual)
// ---------------------------------------------------------------------------

/// Bilingual help text shown in the right pane for a wizard stage.
pub struct StageHelp {
    #[allow(dead_code)]
    pub title_en: &'static str,
    #[allow(dead_code)]
    pub title_th: &'static str,
    pub body_en: &'static str,
    pub body_th: &'static str,
}

impl StageHelp {
    pub fn body(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => self.body_en,
            Lang::Th => self.body_th,
        }
    }
}

pub const HELP_WORKSPACE_PATH: StageHelp = StageHelp {
    title_en: "What is a Workspace?",
    title_th: "Workspace คืออะไร?",
    body_en: "\
The workspace is the root folder that holds all your agents,\n\
their configuration, memory, and shared task list.\n\n\
You will open this folder every time you want to use your agents.\n\
Example: ~/bwoc-workspace or ~/Documents/my-agents\n\n\
Press Enter to accept the default, or type a custom path.",
    body_th: "\
Workspace คือโฟลเดอร์หลักที่เก็บ agent ทั้งหมดของคุณ\n\
รวมถึงการตั้งค่า, ความจำของ agent, และ task ร่วมกัน\n\n\
คุณจะกลับมาเปิดโฟลเดอร์นี้ทุกครั้งที่ต้องการใช้งาน agent\n\
ตัวอย่าง: ~/bwoc-workspace หรือ ~/Documents/my-agents\n\n\
กด Enter เพื่อใช้ค่า default หรือพิมพ์ path ที่ต้องการ",
};

pub const HELP_WORKSPACE_MODE: StageHelp = StageHelp {
    title_en: "Workspace Mode",
    title_th: "โหมด Workspace",
    body_en: "\
Fleet (team): best if you plan to create multiple agents.\n\
Each agent has its own role, and they can collaborate as a team.\n\
This is the default mode and recommended for most users.\n\n\
Single-agent: best if you only want one agent.\n\
The workspace has a simpler structure with no fleet system.",
    body_th: "\
Fleet (ทีม): เหมาะถ้าคุณวางแผนจะสร้าง agent หลายตัว\n\
Agent แต่ละตัวมีหน้าที่ต่างกัน และสามารถทำงานร่วมกันเป็นทีมได้\n\
นี่คือโหมด default ที่แนะนำสำหรับผู้ใช้ส่วนใหญ่\n\n\
Single-agent: เหมาะถ้าคุณต้องการแค่ agent เดียวเท่านั้น\n\
Workspace จะมี structure ที่เรียบง่ายกว่า ไม่มีระบบ fleet",
};

pub const HELP_WORKSPACE_RUNTIME: StageHelp = StageHelp {
    title_en: "What is the Runtime?",
    title_th: "Runtime คืออะไร?",
    body_en: "\
The runtime is the subsystem that lets agents actually execute tasks.\n\
It includes a daemon for spawning and monitoring agent processes.\n\n\
Enabled (default): agents can run fully.\n\
You can use 'bwoc spawn' and 'bwoc chat' immediately.\n\n\
Disabled (--no-runtime): workspace is readable but agents cannot run.\n\
Suitable only for inspecting structure or making backups.",
    body_th: "\
Runtime หมายถึงระบบที่ช่วยให้ agent รันงานได้จริง\n\
รวมถึง daemon สำหรับ spawn และ monitor agent process\n\n\
เปิด runtime (default): agent สามารถทำงานได้เต็มที่\n\
สามารถใช้ 'bwoc spawn' และ 'bwoc chat' ได้ทันที\n\n\
ปิด runtime (--no-runtime): workspace เปิดได้แต่ agent รันไม่ได้\n\
เหมาะสำหรับดูโครงสร้างหรือ backup เท่านั้น",
};

pub const HELP_WORKSPACE_LANG: StageHelp = StageHelp {
    title_en: "bwoc CLI language",
    title_th: "ภาษาของ bwoc",
    body_en: "\
This setting controls the language bwoc CLI uses for its own output:\n\
notifications, logs, and command output.\n\n\
English (en): bwoc speaks English — easier to search or share.\n\
Thai (th): bwoc speaks Thai — best for Thai-language users.\n\n\
This value is also stored in the BWOC_LANG environment variable.\n\
(This is separate from the wizard UI language you chose earlier.)",
    body_th: "\
ตัวเลือกนี้กำหนดภาษาที่ bwoc CLI จะใช้แสดงผลข้อความต่าง ๆ\n\
รวมถึงข้อความแจ้งเตือน, log, และ output ของคำสั่ง\n\n\
ไทย (th): bwoc จะพูดไทยเป็นหลัก เหมาะสำหรับผู้ใช้ภาษาไทย\n\
English (en): bwoc จะพูดอังกฤษ เหมาะถ้าต้องการ copy-paste\n\
ไปค้นหาหรือแชร์กับคนอื่น\n\n\
ค่านี้จะถูกเก็บใน BWOC_LANG environment variable ด้วย",
};

pub const HELP_AGENT_NAME: StageHelp = StageHelp {
    title_en: "Agent Name",
    title_th: "ชื่อ Agent",
    body_en: "\
An agent is an AI worker with its own name, role, and configuration.\n\
You can have multiple agents inside the same workspace.\n\n\
Name must be kebab-case: lowercase letters, digits, and hyphens only.\n\
Examples: alpha, my-assistant, code-helper, research-bot\n\n\
BWOC will create the folder agents/agent-<name> in your workspace.",
    body_th: "\
Agent คือ AI worker ที่มีชื่อ, หน้าที่, และการตั้งค่าของตัวเอง\n\
คุณสามารถมีหลาย agent ใน workspace เดียวได้\n\n\
ชื่อต้องเป็น kebab-case: ตัวเล็ก, ตัวเลข, และขีด (-) เท่านั้น\n\
ตัวอย่าง: alpha, my-assistant, code-helper, research-bot\n\n\
BWOC จะสร้างโฟลเดอร์ agents/agent-<ชื่อ> ใน workspace ของคุณ",
};

pub const HELP_AGENT_ROLE: StageHelp = StageHelp {
    title_en: "Agent Role",
    title_th: "หน้าที่ของ Agent",
    body_en: "\
The role is a short description of what this agent is here to do.\n\
It is inserted into the agent's system prompt so the AI knows\n\
who it is and what kind of help it should provide.\n\n\
Examples:\n\
- General assistant\n\
- Python coding expert\n\
- Business data analyst\n\
- Thai content writer",
    body_th: "\
Role คือคำอธิบายสั้น ๆ ว่า agent นี้มีหน้าที่ทำอะไร\n\
ข้อความนี้จะถูกนำไปใส่ใน system prompt ของ agent\n\
ช่วยให้ AI รู้ว่าตัวเองเป็นใครและควรช่วยอะไร\n\n\
ตัวอย่าง:\n\
- ผู้ช่วยทั่วไป\n\
- ผู้เชี่ยวชาญด้านโค้ด Python\n\
- นักวิเคราะห์ข้อมูลธุรกิจ\n\
- ผู้ช่วยเขียนบทความภาษาไทย",
};

pub const HELP_AGENT_MODEL: StageHelp = StageHelp {
    title_en: "What is a Model?",
    title_th: "Model คืออะไร?",
    body_en: "\
The model is the 'brain' of the agent — it processes requests and replies.\n\n\
Larger models (e.g. opus, sonnet, gpt-5.5):\n\
  + Smarter, more detailed responses\n\
  - Slower and more expensive\n\n\
Smaller models (e.g. haiku, mini, 7b):\n\
  + Faster and cheaper\n\
  - Less capable\n\n\
A good starting point is a mid-range model; adjust later as needed.",
    body_th: "\
Model คือ 'สมอง' ของ agent — ตัวที่ประมวลผลและตอบคำถาม\n\n\
Model ใหญ่ (เช่น opus, sonnet, gpt-5.5):\n\
  + ฉลาดกว่า ตอบได้ละเอียดกว่า\n\
  - ช้ากว่าและแพงกว่า\n\n\
Model เล็ก (เช่น haiku, mini, 7b):\n\
  + เร็วกว่า ถูกกว่า\n\
  - ไม่ฉลาดเท่า\n\n\
แนะนำให้เริ่มต้นด้วย model กลาง ๆ ก่อน แล้วปรับทีหลัง",
};

pub const HELP_AGENT_FALLBACK: StageHelp = StageHelp {
    title_en: "What is a Fallback Model?",
    title_th: "Fallback Model คืออะไร?",
    body_en: "\
The fallback is a secondary model bwoc uses automatically when the\n\
primary model is unresponsive or returns an error.\n\n\
Example: use claude-sonnet as primary and claude-haiku as fallback\n\
(faster, kicks in if sonnet fails or times out).\n\n\
Choose 'None' if you do not want to set a fallback right now.\n\
You can add one later through the agent's config file.",
    body_th: "\
Fallback คือ model สำรองที่ bwoc จะใช้โดยอัตโนมัติ\n\
เมื่อ primary model ไม่ตอบสนองหรือเกิดข้อผิดพลาด\n\n\
ตัวอย่าง: ใช้ claude-sonnet เป็น primary\n\
และ claude-haiku เป็น fallback (เร็วกว่า ถ้า sonnet ล่ม)\n\n\
เลือก 'ไม่มี' ถ้ายังไม่อยากตั้ง fallback ตอนนี้\n\
สามารถเพิ่มในภายหลังได้ผ่าน config",
};

pub const HELP_BASE_URL: StageHelp = StageHelp {
    title_en: "What is BaseUrl?",
    title_th: "BaseUrl คืออะไร?",
    body_en: "\
The baseUrl is the URL of the API endpoint BWOC will send requests to.\n\
Used for OpenAI-compatible backends that you host yourself.\n\n\
Examples:\n\
- Ollama local: http://localhost:11434/v1\n\
- LM Studio:    http://localhost:1234/v1\n\
- vLLM server:  http://192.168.1.100:8000/v1\n\n\
The URL must end with /v1 and the server must already be running.\n\
Press Enter to use the default (Ollama local).",
    body_th: "\
BaseUrl คือที่อยู่ (URL) ของ API endpoint ที่ BWOC จะส่ง request ไป\n\
ใช้สำหรับ backend แบบ OpenAI-compatible ที่คุณ host เอง\n\n\
ตัวอย่าง:\n\
- Ollama local: http://localhost:11434/v1\n\
- LM Studio: http://localhost:1234/v1\n\
- vLLM server: http://192.168.1.100:8000/v1\n\n\
URL ต้องลงท้ายด้วย /v1 และ server ต้องรันอยู่ก่อน\n\
กด Enter เพื่อใช้ค่า default (Ollama local)",
};
