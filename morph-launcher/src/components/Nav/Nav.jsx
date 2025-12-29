import { Link } from "react-router-dom";
import "./Nav.scss";

export default function Nav()
{
    return (
        <nav id="nav-container">
            <div className="nav-sub-cat">
                <div>
                    <Link to="/">HOME</Link>
                </div>

                <div>
                    <Link to="/locker">LOCKER</Link>
                </div>

                <div>
                    <Link to="/store">STORE</Link>
                </div>
            </div>

            <div className="nav-sub-cat spe">
                <div>
                    <button>Play</button>
                </div>
            </div>

            <div className="nav-sub-cat">
                <div>
                    <Link to="/settings">⚙️</Link>
                </div>
                <div>
                    <Link to="/close">🛑</Link>
                </div>
            </div>
        </nav>
    )
}