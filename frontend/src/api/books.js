export async function updateBookDetail(book_id, book) {
    await fetch(`api/books/${book_id}`, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(book),
    })
}

export async function getAllTags() {
    return await (await fetch(`api/books_tags`)).json()
}

export async function getAllTypes() {
    return await (await fetch(`api/books_types`)).json()
}

export async function getAllLangs() {
    return await (await fetch(`api/books_langs`)).json()
}

export async function getAllBooks() {
    return (await (await fetch('api/books')).json()).books.map(book => {
        book.tags_set = new Set(book.tags || [])
        book.created_time = new Date(book.created_timestamp * 1e3).toLocaleString()
        book.modified_time = new Date(book.modified_timestamp * 1e3).toLocaleString()
        return book
    })
}

export async function searchBooks(query) {
    const query_url = 'api/books?' + new URLSearchParams({ query })
    return (await (await fetch(query_url)).json()).books.map(book => {
        book.tags_set = new Set(book.tags || [])
        book.created_time = new Date(book.created_timestamp * 1e3).toLocaleString()
        book.modified_time = new Date(book.modified_timestamp * 1e3).toLocaleString()
        return book
    })
}
export async function getBook(book_id) {
    return await (await fetch(`api/books/${book_id}`)).json()
}


export async function deleteBook(book_id) {
    await fetch(`api/books/${book_id}`, {
        method: 'DELETE',
    })
}

export async function setBookProgress(book_id, content_idx, progress) {
    await fetch(`api/user/book_progress/${book_id}`, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            content_idx: content_idx,
            progress: progress,
        }),
    })
}

export async function getBookProgress(book_id) {
    let res = await fetch(`api/user/book_progress/${book_id}`)
    if (res.status == 200) {
        return await res.json()
    }
}

export function getContentURL(book_id, content_idx) {
    return `api/books/${book_id}/contents/${content_idx}`
}

export function getBookCoverURL(book, height) {
    if (book.book_cover) {
        return `api/books/${book.book_id}/book_cover`
    }
    return `https://via.placeholder.com/${height}`
}

export async function uploadBookCover(book_id, file) {
    const formData = new FormData();
    formData.append('avatar', file, file.name);
    await fetch(`api/books/${book_id}/book_cover`, {
        method: 'POST',
        body: formData,
    })
}
