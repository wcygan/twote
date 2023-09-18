import React, {useState, useEffect} from 'react';
import axios from 'axios';

function FetchDataComponent() {
    const [data, setData] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        axios.get('https://api.example.com/data')
            .then(response => {
                setData(response.data);
                setLoading(false);
            })
            .catch(error => {
                setError(error);
                setLoading(false);
            });
    }, []);

    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error: {error.message}</p>;

    return <div>{data.someProperty}</div>; // Adjust based on the structure of your API response
}

export default FetchDataComponent;
