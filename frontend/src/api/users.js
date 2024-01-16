export async function login(username, password) {
    return await fetch(`login/`, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            username: username,
            password: password,
        }),
        credentials: 'include',
    })
}
