import HelloComponent from './components/HelloComponent';
import LoginComponent from './components/LoginComponent';
import CreateAccountComponent from "./components/CreateAccountComponent";
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';


function App() {
    return (
        <Router>
            <Routes>
                <Route path="/" element={<HelloComponent />} />
                <Route path="/login" element={<LoginComponent />} />
                <Route path="/create-account" element={<CreateAccountComponent />} />
            </Routes>
        </Router>
    );
}


export default App;
