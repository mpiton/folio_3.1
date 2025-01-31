export class ToastManager {
  show(type: ToastType, message: string, duration: number) {
    // Supprimer les toasts existants du même type
    this.container.querySelectorAll(`.toast--${type}`).forEach(toast => toast.remove());

    // ... existing code ...

    console.log(`[ToastManager] Creating new toast of type "${type}" with ID "${toast.id}"`);

    // Forcer l'opacité à 1 après 1000ms (durée de la transition)
    setTimeout(() => {
      toast.style.opacity = '1';
      console.log(`[ToastManager] Opacity set to 1 for toast "${toast.id}"`);
    }, 1000);

    // ... existing code ...

    console.log(`[ToastManager] Showing toast "${toast.id}"`);
    setTimeout(() => {
      this.updateToastsPosition();
    }, 100);

    console.log(`[ToastManager] Toast "${toast.id}" added to DOM`);
  }

  hide(toast: HTMLElement) {
    console.log(`[ToastManager] Hiding toast "${toast.id}"`);
    // ... existing code ...
  }

  destroy(toast: HTMLElement) {
    console.log(`[ToastManager] Destroying toast "${toast.id}"`);
    // ... existing code ...
    this.updateToastsPosition();
  }

  private updateToastsPosition() {
    setTimeout(() => {
      const toasts = Array.from(this.container.querySelectorAll('.toast--visible'));
      let cumulativeHeight = 0;

      toasts.forEach((toast, index) => {
        const toastRect = toast.getBoundingClientRect();
        const toastHeight = toastRect.height;
        const toastTop = cumulativeHeight;

        toast.style.setProperty('--toast-offset', `${toastTop}px`);

        console.log(`[ToastManager] Toast ${index} positioned at ${toastTop}px (height: ${toastHeight}px)`);

        cumulativeHeight += toastHeight + 8; // 8px de marge entre les toasts
      });
    }, 200);
  }

  // ... existing code ...
}
