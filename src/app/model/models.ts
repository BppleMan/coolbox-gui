export interface Cool {
    name: string;
    description: string;
    dependencies: string[];
    install_tasks: Task[];
    uninstall_tasks: Task[];
    check_tasks: Task[];
}

export interface Task {
    name: string;
    description: string;
}

export interface CoolListItem {
    item: Cool;
    selected: boolean;
}