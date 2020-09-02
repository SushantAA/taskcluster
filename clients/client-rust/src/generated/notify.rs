#![allow(unused_imports)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/** THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT */
use crate::{Client, Credentials, Retry};
use failure::Error;
use serde_json::Value;
use crate::util::urlencode;

/// Notification Service
///
/// The notification service listens for tasks with associated notifications
/// and handles requests to send emails and post pulse messages.
pub struct Notify (Client);

#[allow(non_snake_case)]
impl Notify {
    pub fn new(root_url: &str, credentials: Option<Credentials>, retry: Option<Retry>) -> Result<Self, Error> {
        Ok(Self(Client::new(root_url, "notify", "v1", credentials, retry)?))
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

    /// Send an Email
    /// 
    /// Send an email to `address`. The content is markdown and will be rendered
    /// to HTML, but both the HTML and raw markdown text will be sent in the
    /// email. If a link is included, it will be rendered to a nice button in the
    /// HTML version of the email
    pub async fn email(&self, payload: &Value) -> Result<(), Error> {
        let method = "POST";
        let path = "email";
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }

    /// Publish a Pulse Message
    /// 
    /// Publish a message on pulse with the given `routingKey`.
    pub async fn pulse(&self, payload: &Value) -> Result<(), Error> {
        let method = "POST";
        let path = "pulse";
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }

    /// Post IRC Message
    /// 
    /// Post a message on IRC to a specific channel or user, or a specific user
    /// on a specific channel.
    /// 
    /// Success of this API method does not imply the message was successfully
    /// posted. This API method merely inserts the IRC message into a queue
    /// that will be processed by a background process.
    /// This allows us to re-send the message in face of connection issues.
    /// 
    /// However, if the user isn't online the message will be dropped without
    /// error. We maybe improve this behavior in the future. For now just keep
    /// in mind that IRC is a best-effort service.
    pub async fn irc(&self, payload: &Value) -> Result<(), Error> {
        let method = "POST";
        let path = "irc";
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }

    /// Post Matrix Message
    /// 
    /// Post a message to a room in Matrix. Optionally includes formatted message.
    /// 
    /// The `roomId` in the scopes is a fully formed `roomId` with leading `!` such
    /// as `!foo:bar.com`.
    /// 
    /// Note that the matrix client used by taskcluster must be invited to a room before
    /// it can post there!
    pub async fn matrix(&self, payload: &Value) -> Result<(), Error> {
        let method = "POST";
        let path = "matrix";
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }

    /// Denylist Given Address
    /// 
    /// Add the given address to the notification denylist. The address
    /// can be of either of the three supported address type namely pulse, email
    /// or IRC(user or channel). Addresses in the denylist will be ignored
    /// by the notification service.
    pub async fn addDenylistAddress(&self, payload: &Value) -> Result<(), Error> {
        let method = "POST";
        let path = "denylist/add";
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }

    /// Delete Denylisted Address
    /// 
    /// Delete the specified address from the notification denylist.
    pub async fn deleteDenylistAddress(&self, payload: &Value) -> Result<(), Error> {
        let method = "DELETE";
        let path = "denylist/delete";
        let query = None;
        let body = Some(payload);

        let resp = self.0.request(method, path, query, body).await?;

        resp.bytes().await?;
        Ok(())
    }

    /// List Denylisted Notifications
    /// 
    /// Lists all the denylisted addresses.
    /// 
    /// By default this end-point will try to return up to 1000 addresses in one
    /// request. But it **may return less**, even if more tasks are available.
    /// It may also return a `continuationToken` even though there are no more
    /// results. However, you can only be sure to have seen all results if you
    /// keep calling `list` with the last `continuationToken` until you
    /// get a result without a `continuationToken`.
    /// 
    /// If you are not interested in listing all the members at once, you may
    /// use the query-string option `limit` to return fewer.
    pub async fn listDenylist(&self, continuationToken: Option<&str>, limit: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let path = "denylist/list";
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }
        let body = None;

        let resp = self.0.request(method, path, query, body).await?;

        Ok(resp.json().await?)
    }
}
