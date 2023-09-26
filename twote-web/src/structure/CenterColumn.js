import React from 'react';
import { Container } from 'react-bootstrap';

function CenterColumn({ children }) {
    return (
        <Container>
            {children}
        </Container>
    );
}

export default CenterColumn;
