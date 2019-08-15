import React from 'react';
import './App.scss';

const App: React.FC = () => {
    return (
        <div className="app">
            {/* <div className="header">
                <div className="icon">
                    <img src="/favicon.ico" alt="Icon"/>
                </div>
                <nav>
                    <a href="#idk">Link</a>
                    <a href="#idk">Link</a>
                    <a href="#idk">Link</a>
                    <a href="#idk">Link</a>
                </nav>
            </div> */}
            <div className="serverList">
                {[{
                    name: "jeff",
                    address: "joe.net",
                    id: 101314092401924
                },
                {
                    name: "jem",
                    address: "pe.com",
                    id: 3626236236236235
                },
                {
                    name: "er",
                    address: "red.blues",
                    id: 24636263646234
                }].map(x => ServerCard({server: x}))}
                <div className="serverCard addServer">
                    +
                </div>
            </div>
        </div>
    );
}

interface IServer {
    name: string;
    address: string;
    id: number;
}

const ServerCard: React.FC<{ server: IServer }> = ({ server }) => {
    return (
        <div className="serverCard">
            <div className="name">
                name: {server.name}
            </div>
            <div className="address">
                address: {server.address}
            </div>
            <div className="id">
                id: {server.id}
            </div>
        </div>
    );
}

export default App;
