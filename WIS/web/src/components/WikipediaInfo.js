import React, { Component } from 'react';
import { Panel } from 'react-bootstrap';

export default class WikipediaInfo extends Component {
  constructor(props) {
    super(props);
    this.updateData = this.updateData.bind(this);

    this.state = {
      time: 0,
      description: '',
      title: '',
    };
  }

  render() {
    return (
      <Panel className={this.props.className}>
        <Panel.Heading>
          {this.state.title} <small className="text-muted">(Source: Wikipedia - {this.state.time} milliseconds)</small>
        </Panel.Heading>
        <Panel.Body>{this.state.description}</Panel.Body>
      </Panel>
    );
  }

  updateData() {
    let start = performance.now();

    fetch(`https://de.wikipedia.org/api/rest_v1/page/summary/${encodeURIComponent(this.props.title)}`)
      .then(response => response.json())
      .then(data => {
        let end = performance.now();
        this.setState({ time: end - start });
        this.setState({ description: data.extract });
        this.setState({ title: data.displaytitle });
      });
  }

  componentDidUpdate(prevProps, prevState) {
    if (this.props.title !== prevProps.title) {
      this.updateData();
    }
  }

  componentDidMount() {
    this.updateData();
  }
}
