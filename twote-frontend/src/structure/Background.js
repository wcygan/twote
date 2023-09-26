import React from 'react';
import 'bootstrap/dist/css/bootstrap.min.css';
import './Background.css';

function Background({ children }) {
    return (
        <div className="bg-light min-vh-100">
            {children}
        </div>
    );
}

export default Background;
