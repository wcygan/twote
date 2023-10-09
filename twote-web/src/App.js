import HomePage from './pages/HomePage';
import LoginComponent from './components/LoginComponent';
import CreateAccountComponent from "./components/CreateAccountComponent";
import {BrowserRouter as Router, Route, Routes} from 'react-router-dom';
import Footer from "./structure/Footer";
import Background from "./structure/Background";
import Header from "./structure/Header";
import CenterColumn from "./structure/CenterColumn";
import ProfilePage from "./pages/ProfilePage";
import './App.css';

function App() {
    return (
        <Router>
            <Background>
                <CenterColumn>
                    <Header/>
                    <div className="Middle-Section">
                        <Routes>
                            <Route path="/" element={<HomePage/>}/>
                            <Route path="/login" element={<LoginComponent/>}/>
                            <Route path="/profile/:id" element={<ProfilePage/>}/>
                            <Route path="/create-account" element={<CreateAccountComponent/>}/>
                        </Routes>
                    </div>
                    <Footer/>
                </CenterColumn>
            </Background>
        </Router>
    );
}

export default App;
