export interface IToastManager {
    show(): void;
    close(): void;
    pause(): void;
    resume(): void;
    destroy(): void;
}

export class ToastManager implements IToastManager {
    private toast: HTMLElement;
    private closeButton: HTMLElement | null;
    private progressBar: HTMLElement | null;
    private duration: number;
    private startTime: number | null;
    private remainingTime: number;
    private dismissTimeout: NodeJS.Timeout | null;

    constructor(toast: HTMLElement) {
        this.toast = toast;
        this.closeButton = toast.querySelector("[data-close-toast]");
        this.progressBar = toast.querySelector(".toast-progress");
        this.duration = parseInt(toast.dataset.duration || "5000", 10);
        this.remainingTime = this.duration;
        this.startTime = null;
        this.dismissTimeout = null;

        if (this.closeButton) {
            this.closeButton.addEventListener(
                "click",
                this.close.bind(this)
            );
        }

        this.toast.addEventListener("mouseenter", this.pause.bind(this));
        this.toast.addEventListener("mouseleave", this.resume.bind(this));

        this.show();
    }

    public show(): void {
        requestAnimationFrame(() => {
            this.toast.classList.add("toast--visible");
            if (this.progressBar) {
                this.progressBar.style.transition = `width ${this.duration}ms linear`;
                this.progressBar.style.width = "0%";
            }
            this.startTime = Date.now();
            this.dismissTimeout = setTimeout(() => {
                this.close();
            }, this.duration);
        });
    }

    public close(): void {
        if (this.dismissTimeout) {
            clearTimeout(this.dismissTimeout);
            this.dismissTimeout = null;
        }

        if (this.startTime) {
            this.remainingTime = Math.max(
                0,
                this.duration - (Date.now() - this.startTime)
            );
        }

        if (this.progressBar) {
            this.progressBar.style.transition = "none";
            const computedWidth = getComputedStyle(
                this.progressBar
            ).width;
            this.progressBar.style.width = computedWidth;
        }

        this.toast.classList.remove("toast--visible");

        setTimeout(() => {
            if (this.toast && this.toast.parentNode) {
                this.toast.remove();
            }
        }, 300);
    }

    public pause(): void {
        if (this.dismissTimeout) {
            clearTimeout(this.dismissTimeout);
            this.dismissTimeout = null;
        }

        if (this.startTime) {
            this.remainingTime = Math.max(
                0,
                this.duration - (Date.now() - this.startTime)
            );
        }

        if (this.progressBar) {
            this.progressBar.style.transition = "none";
            const computedWidth = getComputedStyle(
                this.progressBar
            ).width;
            this.progressBar.style.width = computedWidth;
        }
    }

    public resume(): void {
        if (this.remainingTime > 0) {
            this.startTime = Date.now();

            if (this.progressBar) {
                this.progressBar.style.width = "0%";
                this.progressBar.style.transition = `width ${this.remainingTime}ms linear`;
            }

            this.dismissTimeout = setTimeout(() => {
                this.close();
            }, this.remainingTime);
        } else {
            this.close();
        }
    }

    public destroy(): void {
        if (this.dismissTimeout) {
            clearTimeout(this.dismissTimeout);
        }

        if (this.closeButton) {
            this.closeButton.removeEventListener(
                "click",
                this.close.bind(this)
            );
        }

        this.toast.removeEventListener("mouseenter", this.pause.bind(this));
        this.toast.removeEventListener("mouseleave", this.resume.bind(this));

        if (this.toast && this.toast.parentNode) {
            this.toast.remove();
        }
    }
}

// Rendre ToastManager disponible globalement
declare global {
    interface Window {
        ToastManager: typeof ToastManager;
    }
}

window.ToastManager = ToastManager;
