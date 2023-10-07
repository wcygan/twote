import Cookies from 'js-cookie';

export const MY_USER_ID = "my-user-id"
export const AUTH_TOKEN = "authorization-token"

/**
 * A ClientReadableStream wrapper.
 *
 * @template RESPONSE
 * @implements {ClientReadableStream}
 * @constructor
 * @param {!ClientReadableStream<RESPONSE>} stream
 */
const InterceptedStream = function (stream) {
    this.stream = stream;
};

/** @override */
InterceptedStream.prototype.on = function (eventType, callback) {
    if (eventType === 'error') {
        const newCallback = (response) => {
            if (response.code === 16) {
                // Remove token from cookies
                Cookies.remove(AUTH_TOKEN);

                // Redirect to login page
                window.location.href = "/login";
            }
            callback(response);
        };

        this.stream.on(eventType, newCallback);
    } else {
        this.stream.on(eventType, callback);
    }
    return this;
};

/** @override */
InterceptedStream.prototype.cancel = function () {
    this.stream.cancel();
    return this;
};

/**
 * @constructor
 * @implements {StreamInterceptor}
 */
const TestStreamInterceptor = function () {
};

/** @override */
TestStreamInterceptor.prototype.intercept = function (request, invoker) {
    return new InterceptedStream(invoker(request));
};


class AuthInterceptor {
    constructor() {
    }

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
    streamInterceptors: [authInterceptor, new TestStreamInterceptor()],
}

export default AuthInterceptor;