import HomePage from './components/HomePage';
import LoginComponent from './components/LoginComponent';
import CreateAccountComponent from "./components/CreateAccountComponent";
import {BrowserRouter as Router, Route, Routes} from 'react-router-dom';
import Footer from "./structure/Footer";
import Background from "./structure/Background";
import Header from "./structure/Header";
import CenterColumn from "./structure/CenterColumn";
import ProfilePage from "./pages/ProfilePage";


function App() {
    return (
        <Background>
            <CenterColumn>
                <Header/>
                <div>
                    <Router>
                        <Routes>
                            <Route path="/" element={<HomePage/>}/>
                            <Route path="/login" element={<LoginComponent/>}/>
                            <Route path="/profile" element={<ProfilePage/>}/>
                            <Route path="/create-account" element={<CreateAccountComponent/>}/>
                        </Routes>
                    </Router>
                </div>
                <Footer/>
            </CenterColumn>
        </Background>
    );
}


export default App;
