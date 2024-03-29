<style lang="scss">
    :root {
        font-family: 'Roboto'
    }

    :global(.clippy) {
        position: absolute;
        bottom: 0;
        // !important because something sets these values inside the STYLE attribute...
        top: unset !important;
        left: 0 !important;
    }

    .chat-container {
        height: 500px;
        display: flex;
        flex-direction: column;
    }
</style>
<script lang="ts">
    import ChatHistory from '../components/ChatHistory.svelte';
	import MessageInput from '../components/MessageInput.svelte';
    import { Sender, type ChatMessage, type ScriptMessage } from "../model/ChatMessage";
    import { invoke } from '@tauri-apps/api/tauri'
    import clippy from 'clippyjs';
	
    let messages: ChatMessage[] = [];
    let scriptFromTsv: ScriptMessage[] = [];
    let clippyAgent: any | null = null;

    function addMessageToHistory(message: ChatMessage) {
		messages = [...messages, message];
	}

    function postNextClippyMessage() {
        const scriptMessage = scriptFromTsv.shift();
        if (!scriptMessage) {
            addMessageToHistory({
                sender: Sender.CLIPPY,
                content: '========== No script loaded (or previous script has ended). Please load a new script. =========='.split(" "),
                delay: 0,
            } satisfies ChatMessage);
        } else {
            clippyAgent.stop();
            clippyAgent.play('Processing');
            addMessageToHistory(scriptMessage.chatMessage);
            setTimeout(() => {
                clippyAgent.stop();
                clippyAgent.play(scriptMessage.clippyAction);
            }, randomIntFromInterval(6000, 9000));
        }
    }

    function randomIntFromInterval(min: number, max: number) {
        return Math.floor(Math.random() * (max - min + 1) + min)
    }

    async function loadScript(command: String[]) {
        const loadedTsv = await invoke('load_script', { scriptCommand: command.join(' ') }) satisfies String;
        if (loadedTsv === '') {
            addMessageToHistory({
                sender: Sender.CLIPPY,
                content: 'Could not read said file. Try again.'.split(" "),
                delay: 0,
            } satisfies ChatMessage);
            return;
        }

        loadedTsv.split('\n').filter(line => !line.startsWith('#')).slice(1).map(line => {
            line = line.trim();
            const splitLine = line.split('\t');
            scriptFromTsv.push({
                clippyAction: splitLine[0],
                chatMessage: {
                    sender: Sender.CLIPPY,
                    content: splitLine[1].split(" "),
                    delay: randomIntFromInterval(2500, 4500),
                } satisfies ChatMessage
            } satisfies ScriptMessage);

            messages = [];
        })
    }

    async function handleMessage(event: CustomEvent<ChatMessage>) {
        const textContent = event.detail.content;
        const message = {
            sender: Sender.USER,
            content: textContent,
            delay: 0,
        } satisfies ChatMessage;

        addMessageToHistory(message);

        if (textContent[0] === '/load' && scriptFromTsv.length === 0) {
            await loadScript(textContent);
        }
        postNextClippyMessage();
    }

    function loadClippy() {
        if (clippyAgent) {
            clippyAgent.hide();
        }
        window.CLIPPY_CDN = '/clippy-agents/'
        clippy.load('Clippy', function(agent: any) {
            agent.show();
            clippyAgent = agent;
        });
    }

    async function loadInitMessage() {
        const response = await invoke('obtain_init_message', {}) satisfies String;
        addMessageToHistory({
            sender: Sender.CLIPPY,
            content: response.split(" "),
            delay: 0,
        } satisfies ChatMessage)
    }

    const initPage = (_unused: any) => {
        setTimeout(loadClippy, 0);
        loadInitMessage();
    }
</script>

<div class="chat-container" use:initPage>
    <ChatHistory messages={messages} />
    <MessageInput on:message="{handleMessage}" />
</div>