import { DialogApi, MessageApi } from 'naive-ui'

export * from './proxy'
export * from './User'

export interface Window extends globalThis.Window {
    $loadingBar?: {
        start: () => void
        finish: () => void
        error: () => void
    }
    $message?: MessageApi
    $dialog?: DialogApi
    $notification?: any
}
