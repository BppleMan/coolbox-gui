import {BehaviorSubject} from "rxjs"
import {TaskEvent} from "./event"

export class Cool {
    name: string
    description: string
    state: CoolState
    dependencies: string[]
    install_tasks: Task[]
    uninstall_tasks: Task[]
    check_tasks: Task[]
    selected: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false)
    events: BehaviorSubject<TaskEvent[]> = new BehaviorSubject<TaskEvent[]>([])

    constructor(
        cool: Cool,
    ) {
        this.name = cool.name
        this.description = cool.description
        this.state = format_cool_state(cool.state)
        this.dependencies = cool.dependencies
        this.install_tasks = cool.install_tasks
        this.uninstall_tasks = cool.uninstall_tasks
        this.check_tasks = cool.check_tasks
    }
}

export interface Task {
    name: string;
    description: string;
}

export enum CoolState {
    Ready = "Ready",
    Installed = "Installed",
    Installing = "Installing",
    Uninstalling = "Uninstalling",
    Pending = "Pending",
}

export function format_cool_state(state: CoolState): CoolState {
    switch (state) {
    case CoolState.Ready:
        return CoolState.Ready
    case CoolState.Installed:
        return CoolState.Installed
    case CoolState.Installing:
        return CoolState.Installing
    case CoolState.Uninstalling:
        return CoolState.Uninstalling
    default:
        return CoolState.Pending
    }
}

export interface CoolListItem {
    item: Cool;
    selected: boolean;
}
