import {BehaviorSubject} from "rxjs"
import {TaskEvent} from "./event"

export class Cool {
    name: string;
    description: string;
    state: State;
    dependencies: string[];
    install_tasks: Task[];
    uninstall_tasks: Task[];
    check_tasks: Task[];
    selected: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);
    events: BehaviorSubject<TaskEvent[]> = new BehaviorSubject<TaskEvent[]>([]);

    constructor(
        cool: Cool,
    ) {
        this.name = cool.name
        this.description = cool.description
        this.state = cool.state
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

export enum State {
    Ready = "Ready",
    Installed = "Installed",
    Installing = "Installing",
    Uninstalling = "Uninstalling",
}

export interface CoolListItem {
    item: Cool;
    selected: boolean;
}
