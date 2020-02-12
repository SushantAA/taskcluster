# coding=utf-8
#####################################################
# THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT #
#####################################################
# noqa: E128,E201
from ...aio.asyncclient import AsyncBaseClient
from ...aio.asyncclient import createApiClient
from ...aio.asyncclient import config
from ...aio.asyncclient import createTemporaryCredentials
from ...aio.asyncclient import createSession
_defaultConfig = config


class Notify(AsyncBaseClient):
    """
    The notification service listens for tasks with associated notifications
    and handles requests to send emails and post pulse messages.
    """

    classOptions = {
    }
    serviceName = 'notify'
    apiVersion = 'v1'

    async def ping(self, *args, **kwargs):
        """
        Ping Server

        Respond without doing anything.
        This endpoint is used to check that the service is up.

        This method is ``stable``
        """

        return await self._makeApiCall(self.funcinfo["ping"], *args, **kwargs)

    async def email(self, *args, **kwargs):
        """
        Send an Email

        Send an email to `address`. The content is markdown and will be rendered
        to HTML, but both the HTML and raw markdown text will be sent in the
        email. If a link is included, it will be rendered to a nice button in the
        HTML version of the email

        This method is ``experimental``
        """

        return await self._makeApiCall(self.funcinfo["email"], *args, **kwargs)

    async def pulse(self, *args, **kwargs):
        """
        Publish a Pulse Message

        Publish a message on pulse with the given `routingKey`.

        This method is ``experimental``
        """

        return await self._makeApiCall(self.funcinfo["pulse"], *args, **kwargs)

    async def irc(self, *args, **kwargs):
        """
        Post IRC Message

        Post a message on IRC to a specific channel or user, or a specific user
        on a specific channel.

        Success of this API method does not imply the message was successfully
        posted. This API method merely inserts the IRC message into a queue
        that will be processed by a background process.
        This allows us to re-send the message in face of connection issues.

        However, if the user isn't online the message will be dropped without
        error. We maybe improve this behavior in the future. For now just keep
        in mind that IRC is a best-effort service.

        This method is ``experimental``
        """

        return await self._makeApiCall(self.funcinfo["irc"], *args, **kwargs)

    async def addDenylistAddress(self, *args, **kwargs):
        """
        Denylist Given Address

        Add the given address to the notification denylist. The address
        can be of either of the three supported address type namely pulse, email
        or IRC(user or channel). Addresses in the denylist will be ignored
        by the notification service.

        This method is ``experimental``
        """

        return await self._makeApiCall(self.funcinfo["addDenylistAddress"], *args, **kwargs)

    async def deleteDenylistAddress(self, *args, **kwargs):
        """
        Delete Denylisted Address

        Delete the specified address from the notification denylist.

        This method is ``experimental``
        """

        return await self._makeApiCall(self.funcinfo["deleteDenylistAddress"], *args, **kwargs)

    async def listDenylist(self, *args, **kwargs):
        """
        List Denylisted Notifications

        Lists all the denylisted addresses.

        By default this end-point will try to return up to 1000 addresses in one
        request. But it **may return less**, even if more tasks are available.
        It may also return a `continuationToken` even though there are no more
        results. However, you can only be sure to have seen all results if you
        keep calling `list` with the last `continuationToken` until you
        get a result without a `continuationToken`.

        If you are not interested in listing all the members at once, you may
        use the query-string option `limit` to return fewer.

        This method is ``experimental``
        """

        return await self._makeApiCall(self.funcinfo["listDenylist"], *args, **kwargs)

    async def updateWidgets(self, *args, **kwargs):
        """
        Update the Widgets

        This is a temporary API method to exercise infrastructure support for database
        access and migrations.  It is not advertised and will be removed in a later version.

        Do not call this method.

        This method is ``experimental``
        """

        return await self._makeApiCall(self.funcinfo["updateWidgets"], *args, **kwargs)

    funcinfo = {
        "addDenylistAddress": {
            'args': [],
            'input': 'v1/notification-address.json#',
            'method': 'post',
            'name': 'addDenylistAddress',
            'route': '/denylist/add',
            'stability': 'experimental',
        },
        "deleteDenylistAddress": {
            'args': [],
            'input': 'v1/notification-address.json#',
            'method': 'delete',
            'name': 'deleteDenylistAddress',
            'route': '/denylist/delete',
            'stability': 'experimental',
        },
        "email": {
            'args': [],
            'input': 'v1/email-request.json#',
            'method': 'post',
            'name': 'email',
            'route': '/email',
            'stability': 'experimental',
        },
        "irc": {
            'args': [],
            'input': 'v1/irc-request.json#',
            'method': 'post',
            'name': 'irc',
            'route': '/irc',
            'stability': 'experimental',
        },
        "listDenylist": {
            'args': [],
            'method': 'get',
            'name': 'listDenylist',
            'output': 'v1/notification-address-list.json#',
            'query': ['continuationToken', 'limit'],
            'route': '/denylist/list',
            'stability': 'experimental',
        },
        "ping": {
            'args': [],
            'method': 'get',
            'name': 'ping',
            'route': '/ping',
            'stability': 'stable',
        },
        "pulse": {
            'args': [],
            'input': 'v1/pulse-request.json#',
            'method': 'post',
            'name': 'pulse',
            'route': '/pulse',
            'stability': 'experimental',
        },
        "updateWidgets": {
            'args': [],
            'input': 'v1/widget.json#',
            'method': 'post',
            'name': 'updateWidgets',
            'output': 'v1/widgets.json#',
            'route': '/widgets',
            'stability': 'experimental',
        },
    }


__all__ = ['createTemporaryCredentials', 'config', '_defaultConfig', 'createApiClient', 'createSession', 'Notify']
