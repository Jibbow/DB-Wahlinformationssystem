import React, { Component } from 'react';
import { BeatLoader } from 'react-spinners';

// Props:
// stimmkreis=Number [optional]: if given, shows the result only for one Stimmkreis; if omitted, shows the result for the whole election.
export default class Stimmverteilung extends Component {
  constructor(props) {
    super(props);
    this.updateData = this.updateData.bind(this);

    this.state = {
      time: 0,
      stimmenabsolut: [],
    };
  }

  render() {
    if (this.state.time !== 0) {
      return (
        <div>
          <small className="text-muted">Took {this.state.time} milliseconds</small>
          <table class="table">
            <thead>
              <tr>
                <th scope="col">Partei</th>
                <th scope="col">Anzahl an Gesamtstimmen</th>
              </tr>
            </thead>
            <tbody>
              {this.state.stimmenabsolut.map(v => (
                <tr>
                  <td>{v.PARTEI}</td>
                  <td>{v.GESAMTSTIMMEN}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      );
    } else {
      return (
        <div>
          <div className="spinner">
            <BeatLoader color={'#93dee2'} />
          </div>
        </div>
      );
    }
  }

  updateData() {
    let start = performance.now();
    fetch(`http://localhost:8000/stimmverteilung/${this.props.stimmkreis || ''}/2018`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.setState({ time: end - start });
        this.setState({ stimmenabsolut: data });
      });
  }

  componentDidUpdate(prevProps, prevState) {
    if (this.props.stimmkreis !== prevProps.stimmkreis) {
      this.updateData();
    }
  }

  componentDidMount() {
    this.updateData();
  }
}
