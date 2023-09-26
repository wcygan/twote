import HelloComponent from './components/HelloComponent';
import LoginComponent from './components/LoginComponent';
import CreateAccountComponent from "./components/CreateAccountComponent";

function App() {
  return (
      <div>
        <h1>Hello Request:</h1>
        <HelloComponent />
        <h1>Login Request:</h1>
        <LoginComponent />
        <h1>Create Account Request:</h1>
        <CreateAccountComponent />
      </div>
  );
}

export default App;
