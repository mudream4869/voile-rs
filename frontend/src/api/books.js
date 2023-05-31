export async function updateBookDetail(book_id, book) {
    await fetch(`/api/books/${book_id}`, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(book),
    })
}

export async function getAllTags() {
    return await (await fetch(`/api/books_tags`)).json()
}

export async function getAllTypes() {
    return await (await fetch(`/api/books_types`)).json()
}

export async function getAllBooks() {
    return (await (await fetch('/api/books')).json()).books.map(book => {
        book.tags_set = new Set(book.tags || [])
        return book
    })
}

export async function getBook(book_id) {
    return await (await fetch(`/api/books/${book_id}`)).json()
}

