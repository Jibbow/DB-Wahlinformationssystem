import React, { Component } from 'react';
import { Button, Modal } from 'react-bootstrap';

export class VoteButton extends Component {
  constructor(props, context) {
    super(props, context);

    this.handleShow = this.handleShow.bind(this);
    this.handleClose = this.handleClose.bind(this);

    this.state = {
      show: false,
    };
  }

  render() {
    return (
      <div className={this.props.className}>
        <Button bsSize="large" bsStyle="primary" onClick={this.handleShow}>
          Meine Stimme abgeben
        </Button>

        <Modal show={this.state.show} onHide={this.handleClose}>
          <Modal.Header closeButton>
            <Modal.Title>Meine Stimme abgeben</Modal.Title>
          </Modal.Header>
          <Modal.Body>Hi</Modal.Body>
          <Modal.Footer>
            <Button onClick={this.handleClose}>Wahl durchführen</Button>
          </Modal.Footer>
        </Modal>
      </div>
    );
  }

  handleClose() {
    this.setState({ show: false });
  }

  handleShow() {
    this.setState({ show: true });
  }
}
