<h1>{t}Private messages{/t}</h1>

<div class="account-top-tabs" id="account-top-tabs">
	<a class="active" href="javascript:;" id="am-pm-inbox" onclick="Wikijump.modules.AccountMessagesModule.listeners.inbox(event)">{t}Inbox{/t}</a>
	| <a href="javascript:;" id="am-pm-sent" onclick="Wikijump.modules.AccountMessagesModule.listeners.sent(event)">{t}Sent{/t}</a>
	| <a href="javascript:;" id="am-pm-drafts" onclick="Wikijump.modules.AccountMessagesModule.listeners.drafts(event)">{t}Drafts{/t}</a>
	| <a href="javascript:;" id="am-pm-compose" onclick="Wikijump.modules.AccountMessagesModule.listeners.compose(event)">{t}Compose new{/t}</a>
</div>
<div id="pm-action-area">
	<div class="wait-block">{t}Loading messages...{/t}</div>
</div>

