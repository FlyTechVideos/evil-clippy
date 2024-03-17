<style lang="scss">
    .message-history {
        flex-grow: 1;
        overflow-y: scroll;
        height: 320px;
        display: flex;
        flex-direction: column;
        margin-bottom: 20px;

        &::-webkit-scrollbar {
            display: none;
        }

        .message {
            background-color: red;
            width: fit-content;
            padding: 10px;
            margin-bottom: 20px;
            border-radius: 10px;
            position: relative;
            max-width: 90%;

            &::before{
                content: '';
                position: absolute;
                width: 20px;
                height: 20px;
                bottom: -10px;
            }

            &.user {
                background: linear-gradient(130deg, #914887 20%, #8B257E 77.5%);
                box-shadow: -2px 2px 5px 0px rgba(0,0,0,0.5);
                margin-right: 15px;
                align-self: end;
                color: white;

                &::before {
                    clip-path: polygon(0 0, 100% 0, 100% 100%);
                    background-color: #8b257e;
                    left: unset;
                    right: 0;
                }
            }

            &.clippy {
                background-color: #efffba;
                box-shadow: 2px 2px 5px 0px rgba(0,0,0,0.5);
                margin-left: 15px;

                &::before {
                    clip-path: polygon(0 0, 100% 0, 0 100%);
                    background-color: #efffba;
                    left: 0;
                    right: unset;
                }
            }
        }
    }

    /**
     * ==============================================
     * Dot Flashing
     * ==============================================
     */
     // https://codepen.io/nzbin/pen/GGrXbp
    .dot-flashing {
        position: relative;
        width: 10px;
        height: 10px;
        border-radius: 5px;
        background-color: #777;
        color: #777;
        animation: dot-flashing 1s infinite linear alternate;
        animation-delay: 0.5s;

        margin-left: 15px;
        margin-right: 15px;
    }
    .dot-flashing::before, .dot-flashing::after {
        content: "";
        display: inline-block;
        position: absolute;
        top: 0;
    }
    .dot-flashing::before {
        left: -15px;
        width: 10px;
        height: 10px;
        border-radius: 5px;
        background-color: #777;
        color: #777;
        animation: dot-flashing 1s infinite alternate;
        animation-delay: 0s;
    }
    .dot-flashing::after {
        left: 15px;
        width: 10px;
        height: 10px;
        border-radius: 5px;
        background-color: #777;
        color: #777;
        animation: dot-flashing 1s infinite alternate;
        animation-delay: 1s;
    }

    @keyframes dot-flashing {
        0% {
            background-color: #777;
        }
        50%, 100% {
            background-color: rgba(152, 128, 255, 0.2);
        }
    }
</style>
<script lang="ts">
    import { Sender, type ChatMessage } from "../model/ChatMessage";
    
    export let messages: ChatMessage[];
    $: updateMessages(messages);

    function scrollToBottom() {
        const messageHistory = document.getElementById('message-history');
        if (messageHistory) {
            setTimeout(() => {
                messageHistory.scroll({ top: messageHistory.scrollHeight, behavior: 'smooth' });
            }, 0);
        } else {
            console.error('Message history cannot be found');
        }
    }

    function randomIntFromInterval(min: number, max: number) {
        return Math.floor(Math.random() * (max - min + 1) + min)
    }

    function delay(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    function updateMessages(_messages: ChatMessage[]) {
        scrollToBottom();
        if (messages.length > 0) {
            const lastMessage = messages[messages.length - 1];
            if (lastMessage.delay > 0) {
                setTimeout(async () => {
                    const messageContent = [...lastMessage.content];
                    lastMessage.content = [];
                    lastMessage.delay = 0;
                    while (messageContent.length > 0) {
                        let delayMs = randomIntFromInterval(50, 200);
                        if (Math.random() < 0.05) {
                            delayMs = 500;
                        }
                        await delay(delayMs);
                        lastMessage.content.push(messageContent.shift()!);
                        messages = _messages;
                    }
                }, lastMessage.delay)
            }
        }
    }
</script>

<div class="message-history" id="message-history">
    {#each messages as message}
        <div class="message {Sender[message.sender].toLowerCase()}">
            {#if message.delay === 0}
                {@html message.content.join(' ')}
            {:else}
                <div class="dot-flashing"></div>
            {/if}
        </div>
    {/each}
</div>