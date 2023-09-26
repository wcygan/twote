import HelloComponent from './components/HelloComponent';
import LoginComponent from './components/LoginComponent';
import CreateAccountComponent from "./components/CreateAccountComponent";
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';

function App() {
    return (
        <Router>
            <Switch>
                <Route path="/" exact component={HelloComponent} />
                <Route path="/login" component={LoginComponent} />
                <Route path="/create-account" component={CreateAccountComponent} />
            </Switch>
        </Router>
    );
}


export default App;
