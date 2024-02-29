export type Event<T> = {
    event: string,
    id: number,
    payload: T,
    windowLabel: string
}