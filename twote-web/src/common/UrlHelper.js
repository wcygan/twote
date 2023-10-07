// Enum of routes
export const routes = {
    home: '/',
    login: '/login',
    register: '/register',
    profile: '/profile/:id',
}

const idPlaceholder = ':id';

export function userProfile(id) {
    return routes.profile.replace(idPlaceholder, id);
}