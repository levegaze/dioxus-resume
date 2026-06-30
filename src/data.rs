//! Static bilingual content for the resume sections.
//!
//! TH copy is verbatim from the original `index.html` prototype (source of truth
//! for what's displayed); EN copy is a first-draft translation pending review.
//! Tags/tech names are language-neutral. Curation matches the prototype, not the
//! fuller DATA.md — e.g. only 4 projects, trimmed tag lists.

use crate::i18n::{tx, Tx};

// ---------- experience ----------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExpVariant {
    Normal,
    Alt,
    Flagship,
}

impl ExpVariant {
    pub fn class(self) -> &'static str {
        match self {
            ExpVariant::Normal => "exp-item",
            ExpVariant::Alt => "exp-item alt",
            ExpVariant::Flagship => "exp-item flagship",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SubItem {
    pub role: Tx,
    pub desc: Tx,
    pub tags: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExpEntry {
    pub date: Tx,
    pub role: Tx,
    pub org: Tx,
    pub desc: Tx,
    /// Optional collaborator/author credit line ("" = none).
    pub authors: &'static str,
    /// Certificate image keys resolved in the component ("ieee" …); empty = none.
    pub certs: &'static [&'static str],
    /// Bullet feature highlights rendered as a list under the description; empty = none.
    pub highlights: &'static [Tx],
    pub tags: &'static [&'static str],
    pub variant: ExpVariant,
    pub rust_badge: bool,
    /// Non-empty turns this entry into the expandable (Global House) variant.
    pub sub: &'static [SubItem],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TimelineNode {
    Group(Tx),
    Entry(ExpEntry),
}

pub const TIMELINE: &[TimelineNode] = &[
    TimelineNode::Group(tx("2567 (2024)", "2024")),
    TimelineNode::Entry(ExpEntry {
        date: tx("30 ก.ย. 2567", "Sep 30, 2024"),
        role: tx(
            "งานวิจัย — Multi-Factor Authentication for Web Applications Using JWT and Face Embedding-Based Recognition",
            "Research — Multi-Factor Authentication for Web Applications Using JWT and Face Embedding-Based Recognition",
        ),
        org: tx(
            "IEEE Thailand Section · IEEE SMC Thailand Chapter",
            "IEEE Thailand Section · IEEE SMC Thailand Chapter",
        ),
        desc: tx(
            "รับผิดชอบส่วน Deep Learning ของระบบยืนยันตัวตนด้วย JWT + Face Embedding — นำ pre-trained model มาเทรนต่อ ทำ data augmentation และ hyperparameter tuning เพื่อเพิ่มความแม่นยำ จนได้แนวทาง Face Embedding-Based นำเสนอผลงานในงาน IEEE SMC Student Workshop",
            "Owned the deep-learning side of a JWT + face-embedding authentication system — fine-tuned a pre-trained model and applied data augmentation and hyperparameter tuning to improve accuracy, landing on a face-embedding-based approach. Presented at the IEEE SMC Student Workshop.",
        ),
        authors: "Chaiyanan Sompong, Apiwat Kaiburt, Phakwan Penjamuk, Anuwat Supan, Thanadon Boonphiohat, Phattharaphon Kunmee",
        certs: &["ieee"],
        highlights: &[],
        tags: &["Python", "TensorFlow", "MediaPipe"],
        variant: ExpVariant::Normal,
        rust_badge: false,
        sub: &[],
    }),
    TimelineNode::Group(tx("2568 (2025)", "2025")),
    TimelineNode::Entry(ExpEntry {
        date: tx("3 พ.ค. 2568 · ใช้เวลา 1 วัน", "May 3, 2025 · 1 day"),
        role: tx("Freelance — ระบบ \"นางคราม\"", "Freelance — \"Nangkram\" system"),
        org: tx("งานรับจ้างอิสระ", "Independent contract"),
        desc: tx(
            "พัฒนาระบบจัดการสินค้าให้นักศึกษาคณะครุศาสตร์ สาขาวิชานวัตกรรมและคอมพิวเตอร์ศึกษา — ส่วน admin จัดการสินค้า หมวดหมู่ และรูปภาพ พร้อมระบบโพสต์ infographic แบบ carousel และนับยอดการเข้าชมสินค้า รองรับสองภาษา (TH/EN) ส่งมอบภายใน 1 วัน",
            "Built a product-management system for a student in the Faculty of Education's Innovation and Computer Education program — an admin panel for products, categories, and images, plus an infographic carousel and per-product view counts. Bilingual (TH/EN), delivered in a single day.",
        ),
        authors: "",
        certs: &[],
        highlights: &[],
        tags: &["Node.js", "Express", "Prisma", "MySQL", "React", "Cloudinary"],
        variant: ExpVariant::Normal,
        rust_badge: false,
        sub: &[],
    }),
    TimelineNode::Entry(ExpEntry {
        date: tx("19 พ.ค. — 26 ก.ย. 2568", "May 19 – Sep 26, 2025"),
        role: tx(
            "สหกิจศึกษา — Mobile & Web Developer",
            "Co-op — Mobile & Web Developer",
        ),
        org: tx(
            "บริษัท สยามโกลบอลเฮ้าส์ จำกัด (มหาชน)",
            "Siam Global House PCL",
        ),
        desc: tx(
            "ฝึกงานในฝ่าย Innovation & System Development ระยะเวลา 5 เดือน ได้ลงมือทำ 4 โปรเจกต์จริงเต็มรูปแบบ",
            "A 5-month co-op in the Innovation & System Development team, shipping four full production projects.",
        ),
        authors: "",
        certs: &["global_house", "internship"],
        highlights: &[],
        tags: &[],
        variant: ExpVariant::Alt,
        rust_badge: false,
        sub: GLOBAL_HOUSE_SUB,
    }),
    TimelineNode::Entry(ExpEntry {
        date: tx("7 ต.ค. 2568", "Oct 7, 2025"),
        role: tx(
            "Freelance — ระบบติดตามการใช้งบประมาณ",
            "Freelance — Budget tracking system",
        ),
        org: tx(
            "คณะมนุษยศาสตร์และสังคมศาสตร์ มรภ.สกลนคร",
            "Faculty of Humanities & Social Sciences, SNRU",
        ),
        desc: tx(
            "ระบบติดตามงบประมาณของคณะแบบลำดับชั้น — แต่ละโครงการแบ่งหมวดการใช้งบ มีกิจกรรมย่อย และบันทึกการเบิกจ่ายรายรายการในแต่ละกิจกรรม พร้อมกำหนดผู้เบิก/ผู้เกี่ยวข้องเฉพาะรายโครงการหรือรายกิจกรรม",
            "A hierarchical faculty budget tracker — each project breaks down into spending categories, activities, and per-line disbursements within each activity, with expense users assigned to specific projects or activities.",
        ),
        authors: "",
        certs: &[],
        highlights: &[
            tx(
                "ระบบสิทธิ์ 3 บทบาท — เจ้าหน้าที่ (จัดการโครงการ/กิจกรรม/เบิกจ่าย) · ผู้บริหาร (ดู dashboard + รายละเอียดโครงการ) · แอดมิน (จัดการผู้ใช้งาน)",
                "Three roles — staff (manage projects/activities/disbursements) · executive (dashboard + project detail oversight) · admin (user management)",
            ),
            tx(
                "คุมงบแบบแผน vs ใช้จริง คำนวณยอดคงเหลืออัตโนมัติทั้งรายโครงการ รายกิจกรรม และรายหมวดงบ",
                "Planned-vs-actual budget control with auto-computed remaining balance per project, per activity, and per spending category",
            ),
            tx(
                "Dashboard 3 กราฟ — สรุปงบแผน/ใช้จริง สลับมุมมอง 12 เดือน ↔ 4 ไตรมาส, แนวโน้มการใช้งบสะสม, และกราฟเปรียบเทียบงบประมาณ",
                "Three-chart dashboard — planned/actual summary toggling 12-month ↔ 4-quarter views, cumulative spending trend, and a budget comparison chart",
            ),
            tx(
                "รายงานสรุป — การ์ด KPI (งบตั้งไว้ / ใช้จริง / คงเหลือ / % การใช้ / จำนวนโครงการ) และรายงานแยกตามแหล่งงบประมาณรายปีงบ",
                "Summary reporting — KPI cards (planned / actual / remaining / % used / project count) and reports broken down by budget source per fiscal year",
            ),
            tx(
                "export รายงาน/บันทึกเป็นไฟล์ PDF",
                "Report/record export to PDF",
            ),
            tx(
                "จัดการผู้ใช้งานระบบ (แอดมิน)",
                "System user management (admin)",
            ),
        ],
        tags: &["Node.js", "Prisma", "MySQL", "TanStack Query", "Puppeteer", "Recharts"],
        variant: ExpVariant::Normal,
        rust_badge: false,
        sub: &[],
    }),
    TimelineNode::Group(tx("2568 — 2569 (2025–2026)", "2025–2026")),
    TimelineNode::Entry(ExpEntry {
        date: tx("21 พ.ย. 2568", "Nov 21, 2025"),
        role: tx(
            "Freelance — ระบบฐานข้อมูลผู้มีสิทธิ์ได้รับการฟื้นฟูสมรรถภาพ และระบบยืม-คืนอุปกรณ์เครื่องช่วยความพิการ",
            "Freelance — Rehabilitation-eligibility database & assistive-equipment loan system",
        ),
        org: tx("อบจ. และ รพ.สต. จังหวัดสกลนคร", "Sakon Nakhon PAO & sub-district health centers"),
        desc: tx(
            "รพ.สต. แต่ละแห่งบันทึกคำขอของผู้ป่วยในพื้นที่ (จำนวน ประเภทผู้ป่วย อุปกรณ์ที่ขอ และที่อยู่บ้าน) ส่งเข้าระบบให้ อบจ. รวบรวม พอถึงรอบจัดซื้อประจำปีงบประมาณ อบจ. จะจัดสรรแบ่งจ่ายอุปกรณ์ลงแต่ละ รพ.สต. ว่าให้เท่าไหร่และตรงกับที่ขอแค่ไหน เป็นงานแจกจ่ายข้ามหลายฝ่ายทั่วจังหวัด พร้อม export PDF ใบยืมครุภัณฑ์/วัสดุ และใบสรุปรวมของแต่ละ รพ.สต.",
            "Each health center logs local patients' requests (count, patient type, requested equipment, home address) into the system, the PAO aggregates them, and at the annual budget-procurement cycle the PAO allocates equipment to each center, deciding how much to give and how closely to match what was requested. A distribution workflow spanning many parties across the province, with PDF export of equipment/material loan forms and a per-health-center summary sheet.",
        ),
        authors: "",
        certs: &[],
        highlights: &[
            tx(
                "ระบบสิทธิ์ตามบทบาท 3 ระดับ — ส่วนกลาง (อบจ.) / แอดมินหน่วยบริการ / เจ้าหน้าที่ รพ.สต. แยกเมนูและสิทธิ์รายฟีเจอร์",
                "Three-tier role-based access — central (PAO) / facility admin / facility staff, with per-feature permissions and menus",
            ),
            tx(
                "จัดการ master data ครุภัณฑ์/วัสดุแบบหลายชั้น (หมวดหลัก → หมวดย่อย → รายการ) แยกครุภัณฑ์รายชิ้นที่มีรหัสออกจากวัสดุที่เบิกเป็นจำนวน",
                "Multi-level equipment/material master data (main category → sub-category → item), separating per-unit asset-coded equipment from materials issued by quantity",
            ),
            tx(
                "ทะเบียนประชากร/ผู้ป่วยรายหน่วยบริการ — ค้นหา ตรวจสอบ เพิ่ม-แก้ไข พร้อมกำหนดพื้นที่ให้บริการบนแผนที่",
                "Per-facility population/patient registry — search, verify, add/edit — plus service-area definition on a map",
            ),
            tx(
                "รอบปีงบประมาณ — กำหนดรายการครุภัณฑ์/วัสดุของแต่ละปีงบ และเปิด/ปิดการรับคำขอรายหน่วยบริการ",
                "Fiscal-year rounds — define each year's equipment/material catalogue and open/close request intake per facility",
            ),
            tx(
                "แบบฟอร์มร้องขอครุภัณฑ์/วัสดุจากหน้างาน พร้อมหน้าสรุปรายการสำรวจความต้องการ",
                "Field request forms for equipment/materials plus a demand-survey summary view",
            ),
            tx(
                "รายงานความต้องการรวมทั้งจังหวัดแบบ drill-down: จังหวัด → อำเภอ → รพ.สต.",
                "Province-wide demand reporting with province → district → facility drill-down",
            ),
            tx(
                "ระบบจัดซื้อ — รายงานความต้องการ รายงานสรุปการจัดซื้อ และจัดสรรครุภัณฑ์/วัสดุหลังจัดซื้อลงแต่ละหน่วย",
                "Procurement — demand report, purchase summary, and post-purchase allocation of equipment/materials to each facility",
            ),
            tx(
                "ออกรหัสครุภัณฑ์ (asset code) พร้อม generate barcode ติดอุปกรณ์",
                "Asset-code issuance with barcode generation for tagging equipment",
            ),
            tx(
                "งานยืมครุภัณฑ์ — ทำสัญญายืม จัดการเอกสารสัญญา และแจกจ่ายครุภัณฑ์ลงหน่วยบริการ",
                "Equipment borrowing — loan contracts, contract document management, and distribution to facilities",
            ),
            tx(
                "เบิกวัสดุ (material disbursement) แยก flow จากการยืมครุภัณฑ์",
                "Material disbursement as a separate flow from equipment loans",
            ),
            tx(
                "จัดการโครงการ และระบบเปิด/ปิดการใช้งานรวม",
                "Project management and a global system open/close switch",
            ),
        ],
        tags: &["Node.js", "Prisma", "PostgreSQL", "pdfmake", "bwip-js", "React Leaflet", "TanStack Query"],
        variant: ExpVariant::Flagship,
        rust_badge: false,
        sub: &[],
    }),
    TimelineNode::Entry(ExpEntry {
        date: tx("4 มี.ค. 2569", "Mar 4, 2026"),
        role: tx("Freelance — ระบบ Long Term Care", "Freelance — Long Term Care system"),
        org: tx("รพ.สต. โนนหอม", "Non Hom health center"),
        desc: tx(
            "ระบบเยี่ยมบ้านของ รพ.สต. โนนหอม เขียนด้วย Rust ทั้งระบบ deploy บน Windows Server ที่ resource จำกัดมาก — build เป็น .exe ผ่าน GitHub Actions แล้วรันเป็น Windows service ด้วย WinSW",
            "A home-visit system for the Non Hom health center, written entirely in Rust and deployed on a very resource-constrained Windows Server — built to a .exe via GitHub Actions and run as a Windows service with WinSW.",
        ),
        authors: "",
        certs: &[],
        highlights: &[
            tx(
                "วางโครงสร้างด้วย Clean Architecture + DDD จึงต่อยอดได้จริง — เพิ่มโมดูลคัดกรองไข้เลือดออกบนระบบเยี่ยมบ้านได้โดยไม่ต้องรื้อ และเตรียมสถาปัตยกรรมรองรับ multi-tenant ไว้ล่วงหน้า",
                "Structured with Clean Architecture + DDD, so it genuinely extends — a dengue-screening module was added on top of the home-visit system without a rewrite, and it's already laid out for future multi-tenant",
            ),
            tx(
                "ระบบหลายบทบาท 5 ระดับ — ผู้ดูแล (CG) · อสม. (VHV) · เจ้าหน้าที่ รพ.สต. · ผู้บริหาร · แอดมิน แยกเมนูและสิทธิ์ตามบทบาท",
                "Five-role system — caregiver (CG) · village health volunteer (VHV) · health-center officer · supervisor · admin, with role-scoped menus and permissions",
            ),
            tx(
                "แบบฟอร์มเยี่ยมบ้าน LTC แบบหลายส่วน — สัญญาณชีพ, ประเมิน ADL (คิดคะแนนรวม), คัดกรองสุขภาพ, ประเมินปัญหา/สิ่งแวดล้อม, โภชนาการ-สังคม, การใช้ยา และความพึงพอใจ",
                "Multi-section LTC home-visit assessment — vital signs, ADL assessment (with total score), health screening, problem/environment assessment, nutrition & social, medication, and satisfaction",
            ),
            tx(
                "เติมข้อมูลอัตโนมัติจากการเยี่ยมครั้งล่าสุด (สัญญาณชีพ + ADL) พร้อม badge บอกค่าที่เติมให้ — ลดเวลาเยี่ยมซ้ำ",
                "Auto-prefill from the latest visit (vitals + ADL) with a badge marking filled-in values — speeds up repeat visits",
            ),
            tx(
                "บันทึกพิกัด GPS ขณะลงพื้นที่ และปักตำแหน่งบ้านผู้ป่วยบนแผนที่ (react-leaflet)",
                "GPS capture during field visits and pinning the patient's home on a map (react-leaflet)",
            ),
            tx(
                "อัปโหลดรูปถ่ายพร้อมบีบอัดฝั่ง client ก่อนส่ง (browser-image-compression) — ประหยัด bandwidth/พื้นที่บนเซิร์ฟเวอร์ที่ resource จำกัด",
                "Photo upload with client-side image compression (browser-image-compression) — saving bandwidth/storage on a resource-constrained server",
            ),
            tx(
                "ระบบคัดกรองไข้เลือดออก — แบบฟอร์มคัดกรอง เครื่องจับเวลา tourniquet test แนบรูป และประวัติการคัดกรอง",
                "Dengue screening — screening form, a tourniquet-test timer, image attachment, and screening history",
            ),
            tx(
                "จัดการรายชื่อผู้ป่วย — เพิ่ม/แก้ไข แยกประเภทผู้ป่วย ตัวกรอง และมอบหมายผู้ดูแล (CG)/อสม. (VHV) รายคน",
                "Patient registry — add/edit, patient types, filters, and per-patient assignment of caregiver (CG)/VHV",
            ),
            tx(
                "จัดการข้อมูลกลาง — ผู้ใช้งาน หมู่บ้าน ทะเบียนผู้ดูแล (CG) และ อสม. (VHV)",
                "Central master data — users, villages, and caregiver (CG) & VHV registries",
            ),
            tx(
                "ประวัติการลงพื้นที่/คัดกรอง ทั้งแบบรวมและรายผู้ป่วย",
                "Visit/screening history, both aggregate and per-patient",
            ),
            tx(
                "export รายงานการเยี่ยมบ้านเป็นไฟล์ Excel (.xlsx)",
                "Home-visit report export to Excel (.xlsx)",
            ),
            tx(
                "Dashboard ภาพรวมสำหรับผู้บริหาร พร้อมกราฟชั่วโมงกิจกรรม",
                "Supervisor overview dashboard with an activity-hours chart",
            ),
        ],
        tags: &["Axum", "Tokio", "SQLx", "React", "TypeScript", "TanStack Query", "TanStack Router", "Recharts", "WinSW"],
        variant: ExpVariant::Alt,
        rust_badge: true,
        sub: &[],
    }),
];

const GLOBAL_HOUSE_SUB: &[SubItem] = &[
    SubItem {
        role: tx("djing — ระบบ back-office อีคอมเมิร์ซ", "djing — e-commerce back-office"),
        desc: tx(
            "Dashboard ภาพรวมยอดขาย จัดการสินค้า/SKU จัดการคำสั่งซื้อ คลังสินค้า ออกใบแจ้งหนี้ auto-order และเชื่อมต่อ NocNoc marketplace ผ่าน API",
            "Sales dashboard, product/SKU management, orders, warehouse, invoicing, auto-ordering, and a NocNoc marketplace API integration.",
        ),
        tags: &["Next.js", "MUI", "Express", "Auth0", "Bun"],
    },
    SubItem {
        role: tx("chiq — แพลตฟอร์มหน้าร้านอีคอมเมิร์ซ", "chiq — e-commerce storefront platform"),
        desc: tx(
            "Cart + checkout ผ่าน Stripe, wishlist, order tracking, blog พร้อม rich text editor, AI chat assistant, รองรับหลายภาษา",
            "Cart and Stripe checkout, wishlist, order tracking, a blog with a rich-text editor, an AI chat assistant, and multi-language support.",
        ),
        tags: &["Next.js 15", "Prisma", "Stripe", "Gemini", "Firebase"],
    },
    SubItem {
        role: tx("next-transport — BI Dashboard", "next-transport — BI dashboard"),
        desc: tx(
            "RFM Segmentation แยกประเภทลูกค้าตาม Recency/Frequency/Monetary, Key Account Performance, dynamic menu จาก database พร้อม role-based access",
            "RFM segmentation (recency/frequency/monetary), key-account performance, a database-driven dynamic menu, and role-based access.",
        ),
        tags: &["Next.js", "Elysia.js", "JWT", "OpenTelemetry"],
    },
    SubItem {
        role: tx("gogo-banhouse — ระบบ HR สมัครงาน", "gogo-banhouse — HR recruitment system"),
        desc: tx(
            "Job listing แยก store/office, ฟอร์มสมัครงานพร้อมแนบเรซูเม่, คัดกรอง applicant ด้วย AI, จัดตารางสัมภาษณ์, จัดการข้อมูลพนักงานตาม branch/department",
            "Store/office job listings, applications with résumé upload, AI applicant screening, interview scheduling, and employee data by branch/department.",
        ),
        tags: &["Next.js", "Elysia.js", "Prisma", "shadcn/ui"],
    },
];

// ---------- projects ----------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    NoStd,
}

impl Flag {
    pub fn class(self) -> &'static str {
        match self {
            Flag::NoStd => "proj-flag alt",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Flag::NoStd => "no_std",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Project {
    pub name: &'static str,
    pub flag: Option<Flag>,
    pub rust_badge: bool,
    pub desc: Tx,
    pub highlights: &'static [Tx],
    pub stack: &'static [&'static str],
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "auction-engine",
        flag: None,
        rust_badge: true,
        desc: tx(
            "ระบบประมูลแบบ real-time — แต่ละ auction รันเป็น Tokio actor ของตัวเอง ใช้ event sourcing พร้อม snapshot recovery และ fan-out ผ่าน Redis ข้ามหลาย node การ bid ไม่แตะ database บน hot path เลย",
            "A real-time auction platform — each auction runs as its own Tokio actor, with event sourcing plus snapshot recovery and Redis fan-out across nodes. Bids never touch the database on the hot path.",
        ),
        highlights: &[
            tx(
                "Actor-per-auction — แต่ละ AuctionActor เป็น Tokio task ที่ own state ทั้งหมด bid validate + update ใน actor โดยตรง ไม่มี shared lock",
                "Actor-per-auction — each AuctionActor is a Tokio task that owns all auction state; bids validate and update inside the actor with no shared locks",
            ),
            tx(
                "Hot path ไม่แตะ DB — actor ตอบจาก in-memory แล้ว EventWriter batch flush ไป PostgreSQL แบบ async แยก; state rebuild ด้วย snapshot + event replay",
                "Zero-DB hot path — actor replies from in-memory state; EventWriter batches to PostgreSQL async; state recovers via snapshot + event replay",
            ),
            tx(
                "Cross-node fan-out ผ่าน Redis Pub/Sub — actor publish ครั้งเดียว ทุก node relay เข้า per-node broadcast channel ไปยัง WebSocket sessions",
                "Cross-node fan-out via Redis Pub/Sub — actor publishes once; each node relays into a per-node broadcast channel to all WebSocket sessions",
            ),
        ],
        stack: &["Rust", "Axum", "SQLx", "PostgreSQL", "Redis", "WebSocket", "React", "TypeScript"],
    },
    Project {
        name: "bare-metal-arena",
        flag: Some(Flag::NoStd),
        rust_badge: true,
        desc: tx(
            "Bump-pointer allocator และ linked list ในบัฟเฟอร์ static ขนาด 4KB — ไม่มี OS ไม่มี stdlib ไม่มี crates ใดๆ คอมไพล์ได้ทั้ง x86-64 และ AArch64 แบบ bare metal",
            "A bump-pointer allocator and linked list inside a 4KB static buffer — no OS, no stdlib, no crates. Compiles bare-metal for both x86-64 and AArch64.",
        ),
        highlights: &[
            tx(
                "O(1) bump allocator — align-up แล้ว bump offset เดียว, `arena_new<T>` generic typed placement ลง static buffer ไม่พึ่ง heap หรือ Box",
                "O(1) bump allocator — align-up then single offset bump; `arena_new<T>` places any typed value into the static buffer without heap or Box",
            ),
            tx(
                "Insertion sort ด้วยการ rewire pointer — เรียง linked list โดยไม่ copy ข้อมูลและไม่ allocate node เพิ่มเลย",
                "Pointer-rewiring insertion sort — sorts the linked list by relinking next pointers with zero data copies and zero extra allocation",
            ),
        ],
        stack: &["Rust", "no_std", "unsafe"],
    },
    Project {
        name: "lemuen-shell",
        flag: None,
        rust_badge: false,
        desc: tx(
            "Unix shell เขียนเองด้วย C ตั้งแต่ต้น — process control, pipeline, I/O redirection และ line editor ที่เขียนเอง พร้อม raw terminal mode และ history",
            "A Unix shell written from scratch in C — process control, pipelines, I/O redirection, and a hand-written line editor with raw terminal mode and history.",
        ),
        highlights: &[
            tx(
                "Line editor เขียนเองด้วย raw terminal mode (termios) — arrow key ไล่ history, Ctrl+C/D, persist ลง ~/.lemuen_history ข้ามสมัย ไม่พึ่ง readline library",
                "Hand-written line editor in raw terminal mode (termios) — arrow-key history navigation, Ctrl+C/D, persisted to ~/.lemuen_history across sessions; no readline library",
            ),
            tx(
                "Parser เขียนเองรองรับ pipeline หลายขั้น, I/O redirection (<, >, >>), command chaining (;, &&, ||), background (&) และ variable/tilde expansion",
                "Hand-written parser supporting multi-stage pipelines, I/O redirection (<, >, >>), command chaining (;, &&, ||), background execution, and variable/tilde expansion",
            ),
        ],
        stack: &["C", "POSIX", "termios"],
    },
    Project {
        name: "nca-digits",
        flag: None,
        rust_badge: true,
        desc: tx(
            "Conditional Growing Neural Cellular Automata — ปลูก digit 0-9 จาก seed cell เดียว เขียน BPTT + Adam เอง ไม่พึ่ง ML framework ใดๆ",
            "Conditional Growing Neural Cellular Automata — grows digits 0–9 from a single seed cell, with hand-written BPTT and Adam; no ML framework.",
        ),
        highlights: &[
            tx(
                "BPTT + Adam เขียนเองทั้งหมด — backprop through 16 CA steps, gradient check ผ่าน, ไม่มี Burn / Torch / Candle แม้แต่อันเดียว",
                "Fully hand-written BPTT + Adam — backprop through 16 CA steps, gradient-checked; no Burn, Torch, or any ML framework",
            ),
            tx(
                "Perception layer ใช้ Sobel-x/y + identity → 48-d input, 16-channel grid state, experience replay pool + damage augmentation เพื่อ self-repair",
                "Sobel-x/y + identity perception → 48-d input into a 16-channel grid; experience replay pool + damage augmentation for self-repair training",
            ),
        ],
        stack: &["Rust", "ndarray", "rayon"],
    },
    Project {
        name: "conways-game-of-life",
        flag: None,
        rust_badge: false,
        desc: tx(
            "Game of Life แบบ sparse grid พร้อม custom hasher สไตล์ FxHash สำหรับ key เป็น integer เล็กๆ เร็วกว่า SipHash default ประมาณ 3 เท่าสำหรับงานนี้",
            "A sparse-grid Game of Life with a custom FxHash-style hasher for small integer keys — about 3× faster than the default SipHash for this workload.",
        ),
        highlights: &[
            tx(
                "Sparse HashSet — เก็บเฉพาะ cell ที่มีชีวิต ไม่ allocate array เต็ม grid รองรับ pattern ขนาดใหญ่ได้โดยไม่สิ้นเปลือง memory",
                "Sparse HashSet grid — only live cells stored; no full-grid allocation, handles large patterns without wasted memory",
            ),
            tx(
                "Custom FxHasher (rotate-xor-mul) เขียนเองสำหรับ i32 coordinate key — ไม่พึ่ง crate ภายนอก เร็วกว่า SipHash default ~3 เท่า",
                "Hand-written FxHash-style hasher (rotate-xor-mul) for i32 coordinate pairs — no external crate, ~3× faster than default SipHash",
            ),
        ],
        stack: &["Rust", "macroquad"],
    },
];

// ---------- skills ----------

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Skill {
    pub name: &'static str,
    /// Logo key resolved to an asset in `IconSlot`; `None` → dashed placeholder.
    pub icon: Option<&'static str>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SkillGroup {
    pub label: Tx,
    pub skills: &'static [Skill],
}

const fn skill_i(name: &'static str, icon: &'static str) -> Skill {
    Skill { name, icon: Some(icon) }
}

pub const SKILLS: &[SkillGroup] = &[
    SkillGroup {
        label: tx("ภาษา", "Languages"),
        skills: &[
            skill_i("Rust", "rust"),
            skill_i("TypeScript", "ts"),
            skill_i("JavaScript", "js"),
            skill_i("C", "c"),
            skill_i("SQL", "sql"),
        ],
    },
    SkillGroup {
        label: tx("เฟรมเวิร์ก & ไลบรารี", "Frameworks & Libraries"),
        skills: &[
            skill_i("Axum", "axum"),
            skill_i("Tokio", "tokio"),
            skill_i("SQLx", "sqlx"),
            skill_i("Diesel", "diesel"),
            skill_i("Dioxus", "dioxus"),
            skill_i("Express", "express"),
            skill_i("Prisma", "prisma"),
            skill_i("React", "react"),
            skill_i("Next.js", "nextjs"),
            skill_i("TanStack", "tanstack"),
        ],
    },
    SkillGroup {
        label: tx("ฐานข้อมูล & Infra", "Database & Infra"),
        skills: &[
            skill_i("PostgreSQL", "postgres"),
            skill_i("MySQL", "mysql"),
            skill_i("Redis", "redis"),
            skill_i("Docker", "docker"),
            skill_i("Linux", "linux"),
        ],
    },
    SkillGroup {
        label: tx("AI", "AI"),
        skills: &[skill_i("Claude", "claude"), skill_i("Codex", "codex")],
    },
];
