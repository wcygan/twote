import Cookies from 'js-cookie';

require('grpc-web');
export const AUTH_TOKEN = "authorization-token"

// // The UnaryInterceptor interface is for the promise-based client.
// class MyUnaryInterceptor {
//     intercept(request, invoker) {
//         return invoker(request).then(response => {
//             console.log("hello got code" + response.getCode());
//             return response;
//         });
//     }
// }


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
    console.log("bababooy")
    console.log(eventType)
    if (eventType === 'data') {
        console.log("foo")
        const newCallback = (response) => {
            console.log("boob")
            // Update the response message.
            console.log("code is " + response)
            // Pass along the updated response.
            callback(response);
        };
        // Register the new callback.
        this.stream.on(eventType, newCallback);
    } else {
        // You can also override 'status', 'end', and 'error' eventTypes.
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
const TestStreamInterceptor = function() {};

/** @override */
TestStreamInterceptor.prototype.intercept = function(request, invoker) {
    return new InterceptedStream(invoker(request));
};

//
// const UnauthenticatedResponseInterceptor = function() {
//     this.intercept = function(request, invoker) {
//         return invoker(request)
//             .then((response) => {
//                 console.log("got code" + response.getCode());
//
//                 const token = Cookies.get(AUTH_TOKEN);
//                 console.log("adding token to request: " + token);
//
//                 return response;
//             });
//     };
// };


// /**
//  * @constructor
//  * @implements {UnaryInterceptor}
//  */
// const SimpleUnaryInterceptor = function() {};
//
// /** @override */
// SimpleUnaryInterceptor.prototype.intercept = function(request, invoker) {
//     // Update the request message before the RPC.
//     const reqMsg = request.getRequestMessage();
//     reqMsg.setMessage('[Intercept request]' + reqMsg.getMessage());
//
//     // After the RPC returns successfully, update the response.
//     return invoker(request).then((response) => {
//         // You can also do something with response metadata here.
//         console.log(response.getMetadata());
//
//         console.log("foo")
//
//         // Update the response message.
//         const responseMsg = response.getResponseMessage();
//         responseMsg.setMessage('[Intercept response]' + responseMsg.getMessage());
//
//         return response;
//     });
// };

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
    unaryInterceptors: [authInterceptor, new TestStreamInterceptor()],
    streamInterceptors: [authInterceptor, new TestStreamInterceptor()],
}

export default AuthInterceptor;