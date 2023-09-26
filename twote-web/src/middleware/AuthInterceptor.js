import Cookies from 'js-cookie';

export const AUTH_TOKEN = "authorization-token"

class AuthInterceptor {
    constructor() { }

    intercept(request, invoker) {
        const metadata = request.getMetadata();
        const token = Cookies.get(AUTH_TOKEN);
        console.log("adding token to request: " + token);
        metadata.Authorization = token;
        return invoker(request);
    }
}

const authInterceptor = new AuthInterceptor()

export const authOptions = {
    // Add interceptors that are called on each request
    unaryInterceptors: [authInterceptor],
    streamInterceptors: [authInterceptor],
}

export default AuthInterceptor;