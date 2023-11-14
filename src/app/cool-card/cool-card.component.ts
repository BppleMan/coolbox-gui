import {CommonModule} from "@angular/common"
import {ChangeDetectorRef, Component, Input, OnInit, ViewChild} from "@angular/core"
import {MatButtonModule} from "@angular/material/button"
import {MatCheckboxModule} from "@angular/material/checkbox"
import {MatRippleModule} from "@angular/material/core"
import {MatDividerModule} from "@angular/material/divider"
import {MatExpansionModule, MatExpansionPanel} from "@angular/material/expansion"
import {MatIconModule} from "@angular/material/icon"
import {MatProgressBarModule} from "@angular/material/progress-bar"
import {MatStepper, MatStepperModule} from "@angular/material/stepper"
import {HIGHLIGHT_OPTIONS, HighlightModule} from "ngx-highlightjs"
import {BehaviorSubject} from "rxjs"
import {Cool, CoolState, format_cool_state} from "../model/models"
import {CoolService} from "../service/cool.service"

const ActionMap = new Map<CoolState, string>([
    [CoolState.Ready, "Install"],
    [CoolState.Installed, "Uninstall"],
    [CoolState.Installing, "Installing"],
    [CoolState.Uninstalling, "Uninstalling"],
    [CoolState.Pending, "Pending"],
])

@Component({
    selector: "app-cool-card",
    standalone: true,
    imports: [CommonModule, MatButtonModule, MatCheckboxModule, MatExpansionModule, MatIconModule, MatStepperModule, MatRippleModule, HighlightModule, MatDividerModule, MatProgressBarModule],
    templateUrl: "./cool-card.component.html",
    styleUrls: ["./cool-card.component.scss"],
    providers: [
        {
            provide: HIGHLIGHT_OPTIONS,
            useValue: {
                coreLibraryLoader: () => import("highlight.js/lib/core"),
                languages: {
                    typescript: () => import("highlight.js/lib/languages/typescript"),
                    bash: () => import("highlight.js/lib/languages/bash"),
                },
            },
        },
    ],
})
export class CoolCardComponent implements OnInit {
    @ViewChild("expansionPanel") panel!: MatExpansionPanel
    @ViewChild("stepper") stepper!: MatStepper
    @Input() cool!: Cool
    coolState$: BehaviorSubject<CoolState> = new BehaviorSubject<CoolState>(CoolState.Pending)
    actionState: BehaviorSubject<string> = new BehaviorSubject<string>(ActionMap.get(CoolState.Pending)!)
    expanded = false
    progress: number = 0
    currentProgressStep: number = 0
    progressAnimationId: number = 0
    taskConsoleMessages: string[] = []

    constructor(private cool_service: CoolService, private cdr: ChangeDetectorRef) {
    }

    ngOnInit() {
        // fetch cool from backend
        this.cool_service.check_cool([this.cool.name]).then((coolStates: CoolState[]) => {
            console.log(`[${this.cool.name}] state:`, format_cool_state(coolStates[0]))
            this.coolState$.next(format_cool_state(coolStates[0]))
        })

        this.coolState$.subscribe((state) => {
            this.actionState.next(ActionMap.get(state)!)
        })

        // subscribe cool.events to update the progress bar
        this.cool.events.subscribe((events) => {
            if (events.length > 0) {
                let last_event = events[events.length - 1]
                // this.taskConsoleMessages[last_event.task_index] += last_event.message.message + "\n"
                if (!this.taskConsoleMessages[last_event.task_index]) {
                    this.taskConsoleMessages[last_event.task_index] = ""
                }
                // need new line every time
                this.taskConsoleMessages[last_event.task_index] += last_event.message.message + "\n"

                // change stepper if necessary
                // 倒数第二个event（如果存在的话）
                if (events.length > 1) {
                    const last_second_event = events[events.length - 2]
                    if (last_event.task_index > last_second_event.task_index) {
                        this.stepper.next()
                    }
                }

                this.cdr.detectChanges()
                setTimeout(() => {
                    const element = document.getElementById(this.cool.name + "_" + last_event.task_index)
                    console.log("element", element)
                    element && (element.scrollTop = element.scrollHeight - 40)
                }, 100)
                // the prgress bar reaches 100% only when the last event.tasl_state is "Finished"
                // first we should know how much this.cool's install_tasks.length is, then we can calculate the progress
                // basic logic is, if the length is only one, the progress bar animate from 0% to 90% in a proper speed, then wait for the last event to animate from 90% to 100%
                // in other situation, the install_tasks.length is greater than 1, we take event.task_index into calculation
                // basically the progress bar animate from 0% to ((task_index + 1) / install_tasks.length - 10)% in a proper speed

                // TODO we should use a better algorithm to calculate the progress
                if (last_event.task_state == "Finished") {
                    // whatever current animation is, cancel it, and smooth animate to 100%
                    this.animateProgress(100)
                    // TODO should we fetch cool state again?
                    this.cool_service.check_cool([this.cool.name]).then((coolStates: CoolState[]) => {
                        console.log("cool detail", coolStates[0])
                        this.coolState$.next(coolStates[0])
                    })
                } else {
                    let progressStep = (last_event.task_index + 1) / this.cool.install_tasks.length * 100 - 10
                    // should animate from current progres to finalProgress in a proper speed using animation frame
                    if (this.currentProgressStep != progressStep) {
                        this.animateProgress(progressStep)
                    }
                }

            }
        })
    }

    doAction(event: MouseEvent) {
        if (this.coolState$.value == CoolState.Ready) {
            this.install(event)
        } else if (this.coolState$.value == CoolState.Installed) {
            this.uninstall(event)
        }
    }

    animateProgress(to: number) {
        // TODO
        if (this.progressAnimationId) {
            clearInterval(this.progressAnimationId)
        }
        const initialProgress = this.progress
        // should calculate the proper speed, which means the interval is dynamic due to the distance between initialProgress and to
        // if the distance is too large, the interval should be larger
        // if the distance is too small, the interval should be smaller

        const distance = to - initialProgress
        const step = Math.ceil(distance / 10)
        console.log("step", step)
        this.progressAnimationId = setInterval(() => {
            if (this.progress < to) {
                this.progress += step
                this.cdr.detectChanges()
            } else {
                clearInterval(this.progressAnimationId)
            }
        }, 200)
    }


    get currentTask() {
        if (this.cool.events.value.length > 0) {
            let last_event = this.cool.events.value[this.cool.events.value.length - 1]
            return this.cool.install_tasks[last_event.task_index]
        }
        return null
    }

    toggle_select(event: MouseEvent) {
        event.preventDefault()
        event.stopPropagation()
        this.cool.selected.next(!this.cool.selected.value)
    }

    toggle_panel(event: MouseEvent) {
        console.log("toggle_selected")
        event.preventDefault()
        event.stopPropagation()
        this.panel.toggle()
    }

    setOpened() {
        this.expanded = true
    }

    setClosed() {
        this.expanded = false
    }

    copy(content: string) {
        console.log("copy", content)
        navigator.clipboard.writeText(content)
        // TODO we should show toast here to indicate the copy action
    }

    install(event: MouseEvent) {
        event.stopPropagation()
        this.cool_service.install_cool([this.cool]).then()
    }

    uninstall(event: MouseEvent) {
        event.stopPropagation()
        this.cool_service.uninstall_cool([this.cool]).then()
    }
}
