export class FormatDate {
    // zero indexed month
    static dateToDocumentId(date: Date) { 
        const year = date.getFullYear();
        const month = String(date.getMonth()).padStart(2, '0'); // No +1 here
        const day = String(date.getDate()).padStart(2, '0');
        return `${year}-${month}-${day}`;
    }
}