export function colorTopic(id: number) {
    if (id === 0) {
        return 'primary';
    } else if (id === 2) {
        return 'success';
    } else if (id === 1) {
        return 'warning';
    } else if (id === 3) {
        return 'danger';
    } else {
        return 'info';
    }
}