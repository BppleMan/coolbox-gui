export interface TaskEvent {
    cool_name: string;
    task_name: string;
    task_index: number;
    task_state: TaskState;
    message: Message;
}

export enum TaskState {
    Running = "Running",
    Finished = "Finished",
    Failed = "Failed",
}

export interface Message {
    message_type: MessageType,
    message: string;
}

export enum MessageType {
    Info = "Info",
    Warn = "Warn",
    Error = "Error",
}
