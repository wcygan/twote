import Cookies from 'js-cookie';

export const AUTH_TOKEN = "authorization-token"

class AuthInterceptor {
    constructor() { }

    intercept(request, invoker) {
        const metadata = request.getMetadata();
        console.log("adding token to request: " + Cookies.get(AUTH_TOKEN));
        metadata.Authorization = 'Bearer ' + Cookies.get(AUTH_TOKEN);
        return invoker(request);
    }
}

const authInterceptor = new AuthInterceptor()

export const authOptions = {
    // Add interceptors that are called on each request
    unaryInterceptors: [authInterceptor],
    streamInterceptors: [authInterceptor],
    // Ensure that credentials are included with each request
    withCredentials: true,
    credentials: 'include'
}

export default AuthInterceptor;