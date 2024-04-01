<style lang="scss">
    .chatbox {
        box-shadow: 2px 2px 5px 0px rgba(0,0,0,0.5);
        border: 2px solid #52514f;
        border-radius: 20px;
        padding: 10px;
        background-color: #3b3a38;
        color: #d6d6d6;

        display: flex;
        justify-content: space-between;
        column-gap: 10px;
        margin-left: auto;
        width: 640px;

        .chatbox-text {
            width: 95%;
            align-self: center;
            padding: 5px;
            outline: none;
        }

        .send-button {
            fill: #52514f;
            border: none;
            background: none;
            width: 30px;
            height: 30px;
            border-radius: 8px;

            &:enabled {
                fill: #daa7ba;
            }

            &:hover:enabled, &:focus:enabled {
                background: grey;
            }
        }
    }

    [contenteditable=true]:empty:before{
        content: attr(placeholder);
        pointer-events: none;
        display: block;
    }
</style>
<script lang="ts">
	import SendIcon from '../components/SendIcon.svelte';
    import { createEventDispatcher } from 'svelte'

    let disabled = true;

    const dispatch = createEventDispatcher()

    function sendMessage() {
        const chatboxTextBox = document.getElementById("chatbox-text");
        if (!chatboxTextBox) {
            console.error('Chatbox cannot be found');
            return;
        }
        if (chatboxTextBox.innerHTML) {
            dispatch('message', {
                content: chatboxTextBox.innerHTML.split(" ")
            })
            chatboxTextBox.innerHTML = "";
        }
        chatboxTextBox.focus();
    }

    function handleEnterKeypress(event: KeyboardEvent) {
        if (!event.shiftKey && event.key === 'Enter') {
            event.preventDefault();
            sendMessage();
        }
    }

	function disableButtonIfNoText() {
		const chatboxTextBox = document.getElementById("chatbox-text");
        disabled = (chatboxTextBox?.textContent?.length ?? 0) === 0;
	}
</script>

<div class="chatbox">
    <div
        id="chatbox-text"
        class="chatbox-text"
        contenteditable="true"
        placeholder="Ask Clippy anything..."
        role="textbox"
        tabindex="0"
        on:keydown={handleEnterKeypress}
        on:keyup={disableButtonIfNoText}>
    </div>
    <button class="send-button" disabled={disabled} on:click="{sendMessage}">
        <SendIcon />
    </button>
</div>