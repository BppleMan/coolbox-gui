import {CheckTask, InstallTask, Task} from "./task"

export class Cool {
    constructor(
        public name: string,
        public description: string,
        public  dependencies: string[],
        public install_tasks: Task[],
        public uninstall_tasks: Task[],
        public check_tasks: Task[],
    ) {
    }
}

export interface ICool {
    name: string;
    description: string;
    dependencies: string[];
    install_tasks: { 0: {[k: string]: Task} };
    uninstall_tasks: { 0: Task[] };
    check_tasks: { 0: Task[] };
}
