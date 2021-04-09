mod auto_detect_source_language_config;
mod keyword_recognition_model;
mod recognition_event;
mod session_event;
mod source_language_config;
mod speech_config;
mod speech_recognition_canceled_event;
mod speech_recognition_event;
mod speech_recognition_result;
mod speech_recognizer;
mod speech_synthesis_event;
mod speech_synthesis_result;

// re-export structs directly under speech module
pub use self::auto_detect_source_language_config::AutoDetectSourceLanguageConfig;
pub use self::keyword_recognition_model::KeywordRecognitionModel;
pub use self::recognition_event::RecognitionEvent;
pub use self::session_event::SessionEvent;
pub use self::source_language_config::SourceLanguageConfig;
pub use self::speech_config::SpeechConfig;
pub use self::speech_recognition_canceled_event::SpeechRecognitionCanceledEvent;
pub use self::speech_recognition_event::SpeechRecognitionEvent;
pub use self::speech_recognition_result::SpeechRecognitionResult;
pub use self::speech_recognizer::SpeechRecognizer;
pub use self::speech_synthesis_event::SpeechSynthesisEvent;
pub use self::speech_synthesis_result::SpeechSynthesisResult;
