export async function uploadAvatar(avatar_file) {
    const formData = new FormData();
    formData.append('avatar', avatar_file, avatar_file.name);
    await fetch(`api/config/user/avatar`, {
        method: 'POST',
        body: formData,
    })
}

export async function updateUserConfig(user_config) {
    await fetch(`api/config/user`, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(user_config),
    })
}

export async function updateUserPassword(old_password, new_password) {
    return await fetch(`api/config/user/password`, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            old_password: old_password,
            new_password: new_password,
        }),
    })
}

export async function awaitUserConfig() {
    return await fetch(`api/config/user`);
}

export async function getUserConfig() {
    return await (await fetch(`api/config/user`)).json();
}

export async function getSystemConfig() {
    return await (await fetch(`api/config/system`)).json();
}
