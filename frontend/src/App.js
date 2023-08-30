import { Routes, Route } from 'react-router-dom'
import Home from './pages/Home';
import './styles/App.css';
import Test from './pages/Test';
import List from './pages/List';

function App() {
  return (
    <div className="App">
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path='/list' element={<List />} />
        <Route path="/test" element={<Test />} />
      </Routes>
    </div>
  );
}

export default App;
