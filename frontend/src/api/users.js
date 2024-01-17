export async function login(password) {
    return await fetch(`login/`, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            password: password,
        }),
        credentials: 'include',
    })
}
