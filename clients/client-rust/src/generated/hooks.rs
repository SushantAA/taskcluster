#![allow(unused_imports)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/** THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT */
use crate::{Client, Credentials, Retry};
use failure::Error;
use serde_json::Value;
use crate::util::urlencode;

/// Hooks Service
///
/// The hooks service provides a mechanism for creating tasks in response to events.
///
pub struct Hooks (Client);

#[allow(non_snake_case)]
impl Hooks {
    pub fn new(root_url: &str, credentials: Option<Credentials>, retry: Option<Retry>) -> Result<Self, Error> {
        Ok(Self(Client::new(root_url, "hooks", "v1", credentials, retry)?))
    }

    /// Ping Server
    /// 
    /// Respond without doing anything.
    /// This endpoint is used to check that the service is up.
    pub async fn ping(&self) -> Result<(), Error> {
        let method = "GET";
        let path = "ping";
        let query = None;
        let body = None;

        let resp = self.0.request(method, path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }

    /// List hook groups
    /// 
    /// This endpoint will return a list of all hook groups with at least one hook.
    pub async fn listHookGroups(&self) -> Result<Value, Error> {
        let method = "GET";
        let path = "hooks";
        let query = None;
        let body = None;

        let resp = self.0.request(method, path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// List hooks in a given group
    /// 
    /// This endpoint will return a list of all the hook definitions within a
    /// given hook group.
    pub async fn listHooks(&self, hookGroupId: &str) -> Result<Value, Error> {
        let method = "GET";
        let path = format!("hooks/{}", urlencode(hookGroupId));
        let query = None;
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Get hook definition
    /// 
    /// This endpoint will return the hook definition for the given `hookGroupId`
    /// and hookId.
    pub async fn hook(&self, hookGroupId: &str, hookId: &str) -> Result<Value, Error> {
        let method = "GET";
        let path = format!("hooks/{}/{}", urlencode(hookGroupId), urlencode(hookId));
        let query = None;
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Get hook status
    /// 
    /// This endpoint will return the current status of the hook.  This represents a
    /// snapshot in time and may vary from one call to the next.
    /// 
    /// This method is deprecated in favor of listLastFires.
    pub async fn getHookStatus(&self, hookGroupId: &str, hookId: &str) -> Result<Value, Error> {
        let method = "GET";
        let path = format!("hooks/{}/{}/status", urlencode(hookGroupId), urlencode(hookId));
        let query = None;
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Create a hook
    /// 
    /// This endpoint will create a new hook.
    /// 
    /// The caller's credentials must include the role that will be used to
    /// create the task.  That role must satisfy task.scopes as well as the
    /// necessary scopes to add the task to the queue.
    pub async fn createHook(&self, hookGroupId: &str, hookId: &str, payload: &Value) -> Result<Value, Error> {
        let method = "PUT";
        let path = format!("hooks/{}/{}", urlencode(hookGroupId), urlencode(hookId));
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Update a hook
    /// 
    /// This endpoint will update an existing hook.  All fields except
    /// `hookGroupId` and `hookId` can be modified.
    pub async fn updateHook(&self, hookGroupId: &str, hookId: &str, payload: &Value) -> Result<Value, Error> {
        let method = "POST";
        let path = format!("hooks/{}/{}", urlencode(hookGroupId), urlencode(hookId));
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Delete a hook
    /// 
    /// This endpoint will remove a hook definition.
    pub async fn removeHook(&self, hookGroupId: &str, hookId: &str) -> Result<(), Error> {
        let method = "DELETE";
        let path = format!("hooks/{}/{}", urlencode(hookGroupId), urlencode(hookId));
        let query = None;
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }

    /// Trigger a hook
    /// 
    /// This endpoint will trigger the creation of a task from a hook definition.
    /// 
    /// The HTTP payload must match the hooks `triggerSchema`.  If it does, it is
    /// provided as the `payload` property of the JSON-e context used to render the
    /// task template.
    pub async fn triggerHook(&self, hookGroupId: &str, hookId: &str, payload: &Value) -> Result<Value, Error> {
        let method = "POST";
        let path = format!("hooks/{}/{}/trigger", urlencode(hookGroupId), urlencode(hookId));
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Get a trigger token
    /// 
    /// Retrieve a unique secret token for triggering the specified hook. This
    /// token can be deactivated with `resetTriggerToken`.
    pub async fn getTriggerToken(&self, hookGroupId: &str, hookId: &str) -> Result<Value, Error> {
        let method = "GET";
        let path = format!("hooks/{}/{}/token", urlencode(hookGroupId), urlencode(hookId));
        let query = None;
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Reset a trigger token
    /// 
    /// Reset the token for triggering a given hook. This invalidates token that
    /// may have been issued via getTriggerToken with a new token.
    pub async fn resetTriggerToken(&self, hookGroupId: &str, hookId: &str) -> Result<Value, Error> {
        let method = "POST";
        let path = format!("hooks/{}/{}/token", urlencode(hookGroupId), urlencode(hookId));
        let query = None;
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Trigger a hook with a token
    /// 
    /// This endpoint triggers a defined hook with a valid token.
    /// 
    /// The HTTP payload must match the hooks `triggerSchema`.  If it does, it is
    /// provided as the `payload` property of the JSON-e context used to render the
    /// task template.
    pub async fn triggerHookWithToken(&self, hookGroupId: &str, hookId: &str, token: &str, payload: &Value) -> Result<Value, Error> {
        let method = "POST";
        let path = format!("hooks/{}/{}/trigger/{}", urlencode(hookGroupId), urlencode(hookId), urlencode(token));
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }

    /// Get information about recent hook fires
    /// 
    /// This endpoint will return information about the the last few times this hook has been
    /// fired, including whether the hook was fired successfully or not
    pub async fn listLastFires(&self, hookGroupId: &str, hookId: &str) -> Result<Value, Error> {
        let method = "GET";
        let path = format!("hooks/{}/{}/last-fires", urlencode(hookGroupId), urlencode(hookId));
        let query = None;
        let body = None;

        let resp = self.0.request(method, &path, query, body).await?;

        Ok(resp.json().await?)
    }
}
