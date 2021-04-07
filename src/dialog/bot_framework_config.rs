use crate::dialog::DialogServiceConfig;
use crate::error::{convert_err, Result};
use crate::ffi::{
    bot_framework_config_from_authorization_token, bot_framework_config_from_subscription,
    SPXSPEECHCONFIGHANDLE,
};
use crate::speech::SpeechConfig;
use std::ffi::CString;
use std::mem::MaybeUninit;

#[derive(Debug)]
pub struct BotFrameworkConfig {
    // does not make sense to keep handle explicitly since
    // it is the very same handle as in config. Also not sure
    // what would happen if both SmartHandles (handle + config)
    // would try to release the same underlying SPXSPEECHCONFIGHANDLE
    // probably some kind of double-free C error
    // pub handle: SmartHandle<SPXSPEECHCONFIGHANDLE>,
    pub config: SpeechConfig,
}

impl DialogServiceConfig for BotFrameworkConfig {
    fn get_speech_config(&mut self) -> &mut SpeechConfig {
        &mut self.config
    }

    fn get_handle(&self) -> SPXSPEECHCONFIGHANDLE {
        self.config.handle.inner()
    }
}

impl BotFrameworkConfig {
    pub fn from_subscription<S>(subscription_key: S, region: S) -> Result<Self>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let mut handle: SPXSPEECHCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_sub_key = CString::new(subscription_key)?;
            let c_region = CString::new(region)?;
            let ret = bot_framework_config_from_subscription(
                &mut handle,
                c_sub_key.as_ptr(),
                c_region.as_ptr(),
                std::ptr::null(),
            );
            convert_err(ret, "DialogServiceConnector::from_subscription error")?;
            Ok(BotFrameworkConfig {
                //handle: SmartHandle::create("DialogServiceConnector", handle, speech_config_release),
                config: SpeechConfig::from_handle(handle)?,
            })
        }
    }

    pub fn from_subscription_and_bot_id<S>(
        subscription_key: S,
        region: S,
        bot_id: S,
    ) -> Result<Self>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let mut handle: SPXSPEECHCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_sub_key = CString::new(subscription_key)?;
            let c_region = CString::new(region)?;
            let c_bot_id = CString::new(bot_id)?;
            let ret = bot_framework_config_from_subscription(
                &mut handle,
                c_sub_key.as_ptr(),
                c_region.as_ptr(),
                c_bot_id.as_ptr(),
            );
            convert_err(
                ret,
                "DialogServiceConnector::from_subscription_and_bot_id error",
            )?;
            Ok(BotFrameworkConfig {
                config: SpeechConfig::from_handle(handle)?,
            })
        }
    }

    pub fn from_auth_token<S>(authorization_token: S, region: S) -> Result<Self>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let mut handle: SPXSPEECHCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_authorization_token = CString::new(authorization_token)?;
            let c_region = CString::new(region)?;
            let ret = bot_framework_config_from_authorization_token(
                &mut handle,
                c_authorization_token.as_ptr(),
                c_region.as_ptr(),
                std::ptr::null(),
            );
            convert_err(ret, "DialogServiceConnector::from_auth_token error")?;
            Ok(BotFrameworkConfig {
                config: SpeechConfig::from_handle(handle)?,
            })
        }
    }

    pub fn from_auth_token_and_bot_id<S>(
        authorization_token: S,
        region: S,
        bot_id: S,
    ) -> Result<Self>
    where
        S: Into<Vec<u8>>,
    {
        unsafe {
            let mut handle: SPXSPEECHCONFIGHANDLE = MaybeUninit::uninit().assume_init();
            let c_authorization_token = CString::new(authorization_token)?;
            let c_region = CString::new(region)?;
            let c_bot_id = CString::new(bot_id)?;
            let ret = bot_framework_config_from_authorization_token(
                &mut handle,
                c_authorization_token.as_ptr(),
                c_region.as_ptr(),
                c_bot_id.as_ptr(),
            );
            convert_err(
                ret,
                "DialogServiceConnector::from_auth_token_and_bot_id error",
            )?;
            Ok(BotFrameworkConfig {
                config: SpeechConfig::from_handle(handle)?,
            })
        }
    }
}