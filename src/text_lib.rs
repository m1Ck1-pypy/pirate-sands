//------------------------------------------------------------------
// 1. Languages
//------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Language {
    EnglishUS,
    Russian,
    ChineseSimplified,
    ChineseTraditional,
    Spanish,
    French,
    German,
    Japanese,
    Korean,
    Portuguese,
    Other,               // fallback
}

impl Language {
    #[inline]
    fn idx(self) -> usize {
        self as usize
    }

    pub fn detect() -> Self {
        match sys_locale::get_locale().unwrap_or_else(|| "en-US".into()).as_str() {
            "en-US" => Self::EnglishUS,
            "ru-RU" => Self::Russian,
            "zh-CN" => Self::ChineseSimplified,
            "zh-TW" => Self::ChineseTraditional,
            "es-ES" => Self::Spanish,
            "fr-FR" => Self::French,
            "de-DE" => Self::German,
            "ja-JP" => Self::Japanese,
            "ko-KR" => Self::Korean,
            "pt-BR" => Self::Portuguese,
            _       => Self::Other,
        }
    }
}

//------------------------------------------------------------------
// 2. Keys Titles Strings
//------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Titles {
    Header,
    Subheader,
    Slider,
    SoundNotification,
    StartButton,
    AddMinutes,
    Help,
    TimerRunning,
    TimerPaused,
    TimerStopped,
}

impl Titles {
    #[inline]
    fn idx(self) -> usize {
        self as usize
    }
}

//------------------------------------------------------------------
// 3. Translations Table
//    languages × titles
//------------------------------------------------------------------
const TEXTS: [[&str; 10]; 11] = [
    // EnglishUS
    ["Pirate sands Timer",
     "⌛ Working hours ⌛",
     "MINUTES",
     "Sound notification",
     "▶ Start",
     "➕ one minute",
     "Press S: Start/Pause, A: +1m, R: Reset, Q: Quit",
     "Time is passing, FULL FOCUS.",
     "Pause, you need to take a break",
     "The timer is not running"],

    // Russian
    ["Пиратский таймер",
     "⌛ Рабочее время ⌛",
     "МИНУТЫ",
     "Звуковое уведомление",
     "▶ Старт",
     "➕ одна минута",
     "Нажмите S: Старт/Пауза, A: +1 мин, R: Сброс, Q: Выход",
     "Время идёт, ПОЛНАЯ КОНЦЕНТРАЦИЯ.",
     "Пауза, нужно сделать перерыв",
     "Таймер не запущен"],

    // ChineseSimplified
    ["海盗沙地计时器",
     "⌛ 工作时间 ⌛",
     "分钟",
     "声音通知",
     "▶ 开始",
     "➕ 一分钟",
     "按 S：开始/暂停，A：+1 分钟，R：重置，Q：退出",
     "时间流逝，全神贯注。",
     "暂停，你需要休息一下",
     "计时器未运行"],

    // ChineseTraditional
    ["海盜沙地計時器",
     "⌛ 工作時間 ⌛",
     "分鐘",
     "聲音通知",
     "▶ 開始",
     "➕ 一分鐘",
     "按 S：開始/暫停，A：+1 分鐘，R：重置，Q：退出",
     "時間流逝，全神貫注。",
     "暫停，你需要休息一下",
     "計時器未運行"],

    // Spanish
    ["Temporizador de arena pirata",
     "⌛ Horario laboral ⌛",
     "MINUTOS",
     "Notificación sonora",
     "▶ Iniciar",
     "➕ un minuto",
     "Presiona S: Iniciar/Pausar, A: +1 min, R: Reiniciar, Q: Salir",
     "El tiempo pasa, CONCENTRACIÓN TOTAL.",
     "Pausa, necesitas tomar un descanso",
     "El temporizador no está en funcionamiento"],

    // French
    ["Chronomètre des sables pirates",
     "⌛ Heures de travail ⌛",
     "MINUTES",
     "Notification sonore",
     "▶ Démarrer",
     "➕ une minute",
     "Appuyez sur S : Démarrer/Pause, A : +1 min, R : Réinitialiser, Q : Quitter",
     "Le temps passe, CONCENTRATION MAXIMALE.",
     "Pause, vous devez faire une pause",
     "Le chronomètre ne fonctionne pas"],

    // German
    ["Piratensanduhr-Zeitgeber",
     "⌛ Arbeitszeit ⌛",
     "MINUTEN",
     "Tonbenachrichtigung",
     "▶ Start",
     "➕ eine Minute",
     "Drücke S: Start/Pause, A: +1 Min, R: Zurücksetzen, Q: Beenden",
     "Die Zeit vergeht, VOLLE KONZENTRATION.",
     "Pause, du brauchst eine Pause",
     "Der Timer läuft nicht"],

    // Japanese
    ["海賊の砂時計タイマー",
     "⌛ 勤務時間 ⌛",
     "分",
     "サウンド通知",
     "▶ 開始",
     "➕ 1分追加",
     "S: 開始/一時停止、A: +1分、R: リセット、Q: 終了",
     "時間が経過中、全力集中。",
     "一時停止、休憩が必要です",
     "タイマーは動作していません"],

    // Korean
    ["해적 모래 시계 타이머",
     "⌛ 근무 시간 ⌛",
     "분",
     "소리 알림",
     "▶ 시작",
     "➕ 1분 추가",
     "S: 시작/일시정지, A: +1분, R: 재설정, Q: 종료",
     "시간이 흐르는 중, 완전 집중.",
     "일시 중지, 휴식이 필요합니다",
     "타이머가 실행 중이 아닙니다"],

    // Portuguese
    ["Cronômetro de areia pirata",
     "⌛ Horário de trabalho ⌛",
     "MINUTOS",
     "Notificação sonora",
     "▶ Iniciar",
     "➕ um minuto",
     "Pressione S: Iniciar/Pausar, A: +1 min, R: Reiniciar, Q: Sair",
     "O tempo está passando, FOCO TOTAL.",
     "Pausa, você precisa descansar",
     "O cronômetro não está rodando"],

    // Other / fallback
    ["Pirate sands Timer",
     "⌛ Working hours ⌛",
     "MINUTES",
     "Sound notification",
     "▶ Start",
     "➕ +1 min",
     "Press S: Start/Pause, A: +1m, R: Reset, Q: Quit",
     "Time is passing, FULL FOCUS.",
     "Pause, you need to take a break",
     "The timer is not running"],
];

//------------------------------------------------------------------
// 4. Translator
//------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Translator {
    lang: Language,
}

impl Translator {
    pub fn new(lang: Language) -> Self {
        Translator { lang }
    }

    #[inline]
    pub fn translate(&self, key: Titles) -> &'static str {
        TEXTS[self.lang.idx()][key.idx()]
    }
}

//------------------------------------------------------------------
// 5. Constructor default
//------------------------------------------------------------------
impl Default for Translator {
    fn default() -> Self {
        Self::new(Language::detect())
    }
}
