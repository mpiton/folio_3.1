export interface ToastMessage {
  title: string;
  message: string;
}

export interface ToastOptions {
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message: string;
  duration?: number;
}

export class ToastManager {
  private static instance?: ToastManager;
  private container: HTMLElement | null = null;

  private constructor() {
    // Écouter les événements de toast
    document.addEventListener('showToast', ((event: CustomEvent<ToastOptions>) => {
      this.show(event.detail);
    }) as EventListener);
  }

  private getContainer(): HTMLElement {
    if (!this.container) {
      const existingContainer = document.getElementById('toastContainer');
      if (existingContainer) {
        this.container = existingContainer;
      } else {
        this.container = document.createElement('div');
        this.container.id = 'toastContainer';
        this.container.className = 'toast-container';
        document.body.appendChild(this.container);
      }
    }
    return this.container;
  }

  public static getInstance(): ToastManager {
    ToastManager.instance ??= new ToastManager();
    return ToastManager.instance;
  }

  public show(options: ToastOptions): void {
    const { type = 'info', title, message, duration = 5000 } = options;

    const toast = document.createElement('div');
    toast.className = 'toast';
    toast.dataset.type = type;

    const icons = {
      success: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>`,
      error: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg>`,
      warning: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>`,
      info: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>`,
    };

    toast.innerHTML = `
            <div class="toast-icon">${icons[type]}</div>
            <div class="toast-content">
                ${title ? `<h3 class="toast-title">${title}</h3>` : ''}
                <p class="toast-message">${message}</p>
            </div>
            <button class="toast-close" aria-label="Fermer">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
            </button>
        `;

    // Ajouter les styles si nécessaire
    if (!document.getElementById('toast-styles')) {
      const style = document.createElement('style');
      style.id = 'toast-styles';
      style.textContent = `
                .toast-container {
                    position: fixed;
                    bottom: 20px;
                    right: 20px;
                    z-index: 9999;
                    display: flex;
                    flex-direction: column;
                    gap: 8px;
                    pointer-events: none;
                }

                .toast {
                    background: white;
                    border-radius: 8px;
                    padding: 16px;
                    width: 300px;
                    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
                    display: flex;
                    align-items: flex-start;
                    gap: 12px;
                    transform: translateX(120%);
                    transition: transform 0.3s ease-out, opacity 0.3s ease-out;
                    pointer-events: auto;
                    opacity: 0;
                }

                .toast--visible {
                    transform: translateX(0);
                    opacity: 1;
                }

                .toast[data-type="success"] {
                    border-left: 4px solid #10B981;
                    background-color: #ECFDF5;
                }

                .toast[data-type="error"] {
                    border-left: 4px solid #EF4444;
                    background-color: #FEF2F2;
                }

                .toast[data-type="warning"] {
                    border-left: 4px solid #F59E0B;
                    background-color: #FFFBEB;
                }

                .toast[data-type="info"] {
                    border-left: 4px solid #3B82F6;
                    background-color: #EFF6FF;
                }

                .toast-icon {
                    flex-shrink: 0;
                    width: 24px;
                    height: 24px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }

                .toast[data-type="success"] .toast-icon { color: #10B981; }
                .toast[data-type="error"] .toast-icon { color: #EF4444; }
                .toast[data-type="warning"] .toast-icon { color: #F59E0B; }
                .toast[data-type="info"] .toast-icon { color: #3B82F6; }

                .toast-content {
                    flex-grow: 1;
                }

                .toast-title {
                    margin: 0 0 4px;
                    font-weight: 600;
                    color: #1F2937;
                }

                .toast-message {
                    margin: 0;
                    font-size: 14px;
                    color: #4B5563;
                }

                .toast-close {
                    flex-shrink: 0;
                    width: 24px;
                    height: 24px;
                    border: none;
                    background: transparent;
                    padding: 0;
                    cursor: pointer;
                    color: #6B7280;
                    opacity: 0.5;
                    transition: opacity 0.2s;
                }

                .toast-close:hover {
                    opacity: 1;
                }
            `;
      document.head.appendChild(style);
    }

    const container = this.getContainer();
    container.appendChild(toast);

    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        toast.classList.add('toast--visible');
      });
    });

    const closeButton = toast.querySelector('.toast-close');
    if (closeButton) {
      closeButton.addEventListener('click', () => this.close(toast));
    }

    if (duration > 0) {
      setTimeout(() => this.close(toast), duration);
    }
  }

  private close(toast: HTMLElement): void {
    toast.classList.remove('toast--visible');

    const handleRemove = () => {
      if (toast.isConnected) {
        toast.remove();
      }
    };

    toast.addEventListener('transitionend', handleRemove, { once: true });
    setTimeout(handleRemove, 300);
  }
}

// Messages par défaut
export const defaultMessages: Record<string, ToastMessage> = {
  success: {
    title: 'Succès !',
    message: "L'opération a été effectuée avec succès.",
  },
  error: {
    title: 'Erreur !',
    message: "Une erreur s'est produite lors de l'opération.",
  },
  warning: {
    title: 'Attention !',
    message: 'Veuillez vérifier les informations saisies.',
  },
  info: {
    title: 'Information',
    message: 'Une mise à jour est disponible.',
  },
};
