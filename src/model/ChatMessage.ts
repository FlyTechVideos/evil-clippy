export enum Sender {
    USER,
    CLIPPY,
}

export interface ChatMessage {
    sender: Sender,
    content: String[],
    delay: number,
}

export interface ScriptMessage {
    clippyAction: String,
    chatMessage: ChatMessage,
}