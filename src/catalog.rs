/// Static catalog: 7 BWOC backends with Thai descriptions, vendor CLI binary
/// names, and available model lists. Used by the wizard's backend-picker stage.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backend {
    /// The value passed to `bwoc new --backend` — must match bwoc's registry.
    pub id: &'static str,
    /// Display label shown in the selection list.
    pub label: &'static str,
    /// Vendor CLI binary to probe on PATH (`binary_present`).
    pub binary: &'static str,
    /// Short Thai description (2-4 lines) rendered in the right pane.
    pub description: &'static str,
    /// Available model IDs for this backend.
    pub models: &'static [&'static str],
}

pub const BACKENDS: &[Backend] = &[
    Backend {
        id: "claude",
        label: "Claude (Anthropic)",
        binary: "claude",
        description: "\
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
        description: "\
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
        description: "\
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
        description: "\
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
        description: "\
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
        label: "Ollama (local, ฟรี)",
        binary: "ollama",
        description: "\
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
        description: "\
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

/// Thai help text shown in the right pane for each wizard stage (by name).
pub struct StageHelp {
    #[allow(dead_code)]
    pub title: &'static str,
    pub body: &'static str,
}

pub const HELP_WORKSPACE_PATH: StageHelp = StageHelp {
    title: "Workspace คืออะไร?",
    body: "\
Workspace คือโฟลเดอร์หลักที่เก็บ agent ทั้งหมดของคุณ\n\
รวมถึงการตั้งค่า, ความจำของ agent, และ task ร่วมกัน\n\n\
คุณจะกลับมาเปิดโฟลเดอร์นี้ทุกครั้งที่ต้องการใช้งาน agent\n\
ตัวอย่าง: ~/bwoc-workspace หรือ ~/Documents/my-agents\n\n\
กด Enter เพื่อใช้ค่า default หรือพิมพ์ path ที่ต้องการ",
};

pub const HELP_WORKSPACE_MODE: StageHelp = StageHelp {
    title: "โหมด Workspace",
    body: "\
Fleet (ทีม): เหมาะถ้าคุณวางแผนจะสร้าง agent หลายตัว\n\
Agent แต่ละตัวมีหน้าที่ต่างกัน และสามารถทำงานร่วมกันเป็นทีมได้\n\
นี่คือโหมด default ที่แนะนำสำหรับผู้ใช้ส่วนใหญ่\n\n\
Single-agent: เหมาะถ้าคุณต้องการแค่ agent เดียวเท่านั้น\n\
Workspace จะมี structure ที่เรียบง่ายกว่า ไม่มีระบบ fleet",
};

pub const HELP_WORKSPACE_RUNTIME: StageHelp = StageHelp {
    title: "Runtime คืออะไร?",
    body: "\
Runtime หมายถึงระบบที่ช่วยให้ agent รันงานได้จริง\n\
รวมถึง daemon สำหรับ spawn และ monitor agent process\n\n\
เปิด runtime (default): agent สามารถทำงานได้เต็มที่\n\
สามารถใช้ 'bwoc spawn' และ 'bwoc chat' ได้ทันที\n\n\
ปิด runtime (--no-runtime): workspace เปิดได้แต่ agent รันไม่ได้\n\
เหมาะสำหรับดูโครงสร้างหรือ backup เท่านั้น",
};

pub const HELP_WORKSPACE_LANG: StageHelp = StageHelp {
    title: "ภาษาของ bwoc",
    body: "\
ตัวเลือกนี้กำหนดภาษาที่ bwoc CLI จะใช้แสดงผลข้อความต่าง ๆ\n\
รวมถึงข้อความแจ้งเตือน, log, และ output ของคำสั่ง\n\n\
ไทย (th): bwoc จะพูดไทยเป็นหลัก เหมาะสำหรับผู้ใช้ภาษาไทย\n\
English (en): bwoc จะพูดอังกฤษ เหมาะถ้าต้องการ copy-paste\n\
ไปค้นหาหรือแชร์กับคนอื่น\n\n\
ค่านี้จะถูกเก็บใน BWOC_LANG environment variable ด้วย",
};

pub const HELP_AGENT_NAME: StageHelp = StageHelp {
    title: "ชื่อ Agent",
    body: "\
Agent คือ AI worker ที่มีชื่อ, หน้าที่, และการตั้งค่าของตัวเอง\n\
คุณสามารถมีหลาย agent ใน workspace เดียวได้\n\n\
ชื่อต้องเป็น kebab-case: ตัวเล็ก, ตัวเลข, และขีด (-) เท่านั้น\n\
ตัวอย่าง: alpha, my-assistant, code-helper, research-bot\n\n\
BWOC จะสร้างโฟลเดอร์ agents/agent-<ชื่อ> ใน workspace ของคุณ",
};

pub const HELP_AGENT_ROLE: StageHelp = StageHelp {
    title: "หน้าที่ของ Agent",
    body: "\
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
    title: "Model คืออะไร?",
    body: "\
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
    title: "Fallback Model คืออะไร?",
    body: "\
Fallback คือ model สำรองที่ bwoc จะใช้โดยอัตโนมัติ\n\
เมื่อ primary model ไม่ตอบสนองหรือเกิดข้อผิดพลาด\n\n\
ตัวอย่าง: ใช้ claude-sonnet เป็น primary\n\
และ claude-haiku เป็น fallback (เร็วกว่า ถ้า sonnet ล่ม)\n\n\
เลือก 'ไม่มี' ถ้ายังไม่อยากตั้ง fallback ตอนนี้\n\
สามารถเพิ่มในภายหลังได้ผ่าน config",
};

pub const HELP_BASE_URL: StageHelp = StageHelp {
    title: "BaseUrl คืออะไร?",
    body: "\
BaseUrl คือที่อยู่ (URL) ของ API endpoint ที่ BWOC จะส่ง request ไป\n\
ใช้สำหรับ backend แบบ OpenAI-compatible ที่คุณ host เอง\n\n\
ตัวอย่าง:\n\
- Ollama local: http://localhost:11434/v1\n\
- LM Studio: http://localhost:1234/v1\n\
- vLLM server: http://192.168.1.100:8000/v1\n\n\
URL ต้องลงท้ายด้วย /v1 และ server ต้องรันอยู่ก่อน\n\
กด Enter เพื่อใช้ค่า default (Ollama local)",
};
